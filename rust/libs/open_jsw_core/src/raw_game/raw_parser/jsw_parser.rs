use std::collections::HashMap;

use bytebuffer::ByteBuffer;

use super::{RawParser, read_string};
use crate::{
    Result,
    game::GameType,
    raw_game::{
        CellBehaviour, ConveyorDirection, JswRawCell, JswRawGame, JswRawRoom, ROOM_LAYOUT_SIZE,
        ROOM_LAYOUT_WIDTH, RampDirection,
    },
};

const ADDR_OFFSET: usize = 0x8000;
const ROOMS_OFFSET: usize = 0x0C000 - ADDR_OFFSET;
const ROOM_SIZE: usize = 0x100;
const ROOM_COUNT: u8 = 60;
const ROOM_NAME_LENGTH: usize = 0x20;
// const ROOM_LAYOUT_BYTE_COUNT: usize = ROOM_LAYOUT_SIZE / 4;
const CELL_COUNT: usize = 6;
const CELL_LENGTH: usize = 9;
const ITEM_ID: u8 = 6;
const ATTRIBUTE_BUFFER_ADDRESS: u16 = 0x5E00;
const ITEM_TABLE_OFFSET_1: usize = 0xA400 - ADDR_OFFSET;
const ITEM_TABLE_OFFSET_2: usize = 0xA500 - ADDR_OFFSET;
const ITEM_TABLE_LENGTH: usize = 0x100;

pub struct RawJswGame {
    //
}

struct ItemTable {
    /// Map of room number to list of item positions
    items: HashMap<u8, Vec<usize>>,
}

struct ConveyorAndRamp {
    pub conveyor_direction: ConveyorDirection,
    pub conveyor_position: (u16, u16),
    pub conveyor_length: u8,
    pub ramp_direction: RampDirection,
    pub ramp_position: (u16, u16),
    pub ramp_length: u8,
}

impl RawParser for RawJswGame {
    fn extract_game(game_type: GameType, data: &mut ByteBuffer) -> Result<JswRawGame> {
        let item_table = Self::extract_item_table(data)?;

        let raw_game = JswRawGame {
            game_type,
            rooms: Self::extract_rooms(data, &item_table)?,
        };

        Ok(raw_game)
    }
}

impl RawJswGame {
    fn extract_rooms(data: &mut ByteBuffer, item_table: &ItemTable) -> Result<Vec<JswRawRoom>> {
        let mut rooms: Vec<JswRawRoom> = vec![];

        // TODO - work out the file format
        let mut room_no: u8 = 0;
        while room_no < ROOM_COUNT - 1 {
            let room = Self::extract_room(data, room_no, item_table)?;

            rooms.push(room);
            room_no += 1;
        } // 33168 - 399 = 32769

        Ok(rooms)
    }

    fn extract_room(
        data: &mut ByteBuffer,
        room_no: u8,
        item_table: &ItemTable,
    ) -> Result<JswRawRoom> {
        let room_offset = ROOMS_OFFSET + (room_no as usize * ROOM_SIZE);
        data.set_rpos(room_offset);

        // Room name
        data.set_rpos(room_offset + 0x80);
        let raw_name = read_string(data, ROOM_NAME_LENGTH)?;
        let name = raw_name.trim().to_string();

        // Cells
        let cells = Self::extract_cells(data, room_no)?;

        // Layout
        let layout = Self::extract_room_layout(data, room_no, &cells, item_table)?;

        let room = JswRawRoom {
            room_no,
            name,
            layout,
            cells,
        };

        Ok(room)
    }

    fn extract_room_layout(
        data: &mut ByteBuffer,
        room_no: u8,
        _cells: &[JswRawCell],
        item_table: &ItemTable,
    ) -> Result<[u8; ROOM_LAYOUT_SIZE]> {
        // Read conveyor direction, position & length
        // let conveyor_and_ramp = Self::get_conveyor_and_ramp(data, room_no)?;
        // TODO - need to use this info to modify the layout with conveyors and ramps

        let room_offset = ROOMS_OFFSET + (room_no as usize * ROOM_SIZE);
        data.set_rpos(room_offset);

        let mut layout = [0; ROOM_LAYOUT_SIZE];

        let mut byte_in: u8 = 0;
        for (i, byte_out) in layout.iter_mut().take(ROOM_LAYOUT_SIZE).enumerate() {
            let pos_in_cell = i % 4;
            if pos_in_cell == 0 {
                byte_in = data.read_u8()?;
            }

            // Extract the 4 bit pairs from the byte_in that represent the layout cells
            match pos_in_cell {
                0 => *byte_out = (byte_in >> 6) & 0b00000011,
                1 => *byte_out = (byte_in >> 4) & 0b00000011,
                2 => *byte_out = (byte_in >> 2) & 0b00000011,
                3 => *byte_out = byte_in & 0b00000011,
                _ => *byte_out = 0,
            }
        }

        // Read the items from the item table and add them
        if let Some(item_positions) = item_table.items.get(&room_no) {
            for item_pos in item_positions {
                if *item_pos < ROOM_LAYOUT_SIZE {
                    layout[*item_pos] = ITEM_ID;
                }
            }
        }

        Ok(layout)
    }

    fn extract_cells(data: &mut ByteBuffer, room_no: u8) -> Result<Vec<JswRawCell>> {
        let room_offset = ROOMS_OFFSET + (room_no as usize * ROOM_SIZE);

        let mut cells: Vec<JswRawCell> = vec![];

        // Read conveyor direction, position & length
        let conveyor_and_ramp = Self::get_conveyor_and_ramp(data, room_no)?;

        for i in 0..CELL_COUNT {
            data.set_rpos(room_offset + 0xA0 + (i * CELL_LENGTH));
            let attribute = data.read_u8()?;

            // Skip cells with the same attribute, they are unused cells
            if cells.iter().any(|cell| cell.attribute == attribute) {
                continue;
            }

            let sprite = [
                data.read_u8()?,
                data.read_u8()?,
                data.read_u8()?,
                data.read_u8()?,
                data.read_u8()?,
                data.read_u8()?,
                data.read_u8()?,
                data.read_u8()?,
            ];

            let behaviour = Self::get_cell_behaviour(
                i,
                conveyor_and_ramp.conveyor_direction,
                conveyor_and_ramp.ramp_direction,
            );

            let cell = JswRawCell::new(i as u8, attribute, behaviour, sprite);
            cells.push(cell);
        }

        // Add the item cell

        // Get the backgroud colour for the item cell
        let air_attribute = cells[0].attribute;

        data.set_rpos(room_offset + 0x0E1);
        let sprite = [
            data.read_u8()?,
            data.read_u8()?,
            data.read_u8()?,
            data.read_u8()?,
            data.read_u8()?,
            data.read_u8()?,
            data.read_u8()?,
            data.read_u8()?,
        ];
        let attribute = air_attribute | 0x80 | 0x07; // Bright white ink, air paper, no flash
        let cell = JswRawCell::new(ITEM_ID, attribute, CellBehaviour::Item, sprite);
        cells.push(cell);

        Ok(cells)
    }

    fn extract_item_table(data: &mut ByteBuffer) -> Result<ItemTable> {
        let mut item_table = ItemTable {
            items: HashMap::new(),
        };

        data.set_rpos(ITEM_TABLE_OFFSET_1);

        for i in 0..ITEM_TABLE_LENGTH {
            data.set_rpos(ITEM_TABLE_OFFSET_1 + i);
            let byte1 = data.read_u8()?;

            data.set_rpos(ITEM_TABLE_OFFSET_2 + i);
            let byte2 = data.read_u8()?;

            if !(byte1 == 0 && byte2 == 0) {
                // Set item in the item table
                let room_no = byte1 & 0x3F;
                let item_pos: u16 = u16::from_be_bytes([(byte1 >> 7) & 0x01, byte2]);

                let item_positions = item_table.items.entry(room_no).or_default();
                item_positions.push(item_pos as usize);
            }
        }

        Ok(item_table)
    }

    fn get_cell_behaviour(
        cell_no: usize,
        conveyor_direction: ConveyorDirection,
        ramp_direction: RampDirection,
    ) -> CellBehaviour {
        match cell_no {
            0 => CellBehaviour::Air,
            1 => CellBehaviour::Water,
            2 => CellBehaviour::Earth,
            3 => CellBehaviour::Fire,
            4 => {
                if ramp_direction == RampDirection::Left {
                    CellBehaviour::LRamp
                } else {
                    CellBehaviour::RRamp
                }
            }
            5 => {
                if conveyor_direction == ConveyorDirection::Left {
                    CellBehaviour::LConveyor
                } else {
                    CellBehaviour::RConveyor
                }
            }
            _ => CellBehaviour::Air,
        }
    }

    fn get_conveyor_and_ramp(data: &mut ByteBuffer, room_no: u8) -> Result<ConveyorAndRamp> {
        // Store the initial read position
        let initial_rpos = data.get_rpos();

        let room_offset = ROOMS_OFFSET + (room_no as usize * ROOM_SIZE);

        // Read conveyor direction, position & length
        let mut conveyor_direction = ConveyorDirection::Right;
        let mut conveyor_position = (0, 0);
        data.set_rpos(room_offset + 0xD6);
        if data.read_u8()? > 0 {
            conveyor_direction = ConveyorDirection::Left;
        }
        let raw_conveyor_buffer_address = data.read_u16()?;
        if raw_conveyor_buffer_address >= ATTRIBUTE_BUFFER_ADDRESS {
            let raw_conveyor_position = raw_conveyor_buffer_address - ATTRIBUTE_BUFFER_ADDRESS;
            conveyor_position = (
                raw_conveyor_position % ROOM_LAYOUT_WIDTH as u16,
                raw_conveyor_position / ROOM_LAYOUT_WIDTH as u16,
            );
        }
        let conveyor_length = data.read_u8()?;

        // Read ramp direction
        let mut ramp_direction = RampDirection::Right;
        let mut ramp_position = (0, 0);
        data.set_rpos(room_offset + 0x26f);
        if data.read_u8()? > 0 {
            ramp_direction = RampDirection::Left;
        }
        let raw_ramp_buffer_address = data.read_u16()?;
        if raw_ramp_buffer_address >= ATTRIBUTE_BUFFER_ADDRESS {
            let raw_ramp_position = raw_ramp_buffer_address - ATTRIBUTE_BUFFER_ADDRESS;
            ramp_position = (
                raw_ramp_position % ROOM_LAYOUT_WIDTH as u16,
                raw_ramp_position / ROOM_LAYOUT_WIDTH as u16,
            );
        }
        let ramp_length = data.read_u8()?;

        // Reset the read position
        data.set_rpos(initial_rpos);

        Ok(ConveyorAndRamp {
            conveyor_direction,
            conveyor_position,
            conveyor_length,
            ramp_direction,
            ramp_position,
            ramp_length,
        })
    }
}

use bytebuffer::ByteBuffer;

use super::{RawParser, read_string};
use crate::{
    Result,
    game::GameType,
    raw_game::{
        CellBehaviour, ConveyorDirection, JswRawCell, JswRawGame, JswRawRoom, ROOM_LAYOUT_SIZE,
    },
};

const ADDR_OFFSET: usize = 0x8000;
const ROOMS_OFFSET: usize = 0x0B000 - ADDR_OFFSET;
const ROOM_SIZE: usize = 0x400;
const ROOM_COUNT: u8 = 20;
const ROOM_NAME_LENGTH: usize = 0x20;
const CELL_COUNT: usize = 8;
const CELL_LENGTH: usize = 9;

pub struct RawMmGame {
    //
}

impl RawParser for RawMmGame {
    fn extract_game(game_type: GameType, data: &mut ByteBuffer) -> Result<JswRawGame> {
        let raw_game = JswRawGame {
            game_type,
            rooms: Self::extract_rooms(data)?,
        };

        Ok(raw_game)
    }
}

impl RawMmGame {
    fn extract_rooms(data: &mut ByteBuffer) -> Result<Vec<JswRawRoom>> {
        let mut rooms: Vec<JswRawRoom> = vec![];

        // TODO - work out the file format
        let mut room_no: u8 = 0;
        while room_no < ROOM_COUNT {
            let room = Self::extract_room(data, room_no)?;

            rooms.push(room);
            room_no += 1;
        } // 33168 - 399 = 32769

        Ok(rooms)
    }

    fn extract_room(data: &mut ByteBuffer, room_no: u8) -> Result<JswRawRoom> {
        let room_offset = ROOMS_OFFSET + (room_no as usize * ROOM_SIZE);
        data.set_rpos(room_offset);

        // Room name
        data.set_rpos(room_offset + 0x200);
        let raw_name = read_string(data, ROOM_NAME_LENGTH)?;
        let name = raw_name.trim().to_string();

        // Cells
        let cells = Self::extract_cells(data, room_no)?;

        // Layout
        let layout = Self::extract_room_layout(data, room_no, &cells)?;

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
        cells: &[JswRawCell],
    ) -> Result<[u8; ROOM_LAYOUT_SIZE]> {
        let room_offset = ROOMS_OFFSET + (room_no as usize * ROOM_SIZE);
        data.set_rpos(room_offset);

        let mut layout = [0; ROOM_LAYOUT_SIZE];

        for byte_out in layout.iter_mut().take(ROOM_LAYOUT_SIZE) {
            let byte_in = data.read_u8()?;

            *byte_out = match cells.iter().find(|cell| cell.attribute == byte_in) {
                Some(cell) => cell.id,
                None => 0,
            };
        }

        Ok(layout)
    }

    fn extract_cells(data: &mut ByteBuffer, room_no: u8) -> Result<Vec<JswRawCell>> {
        let room_offset = ROOMS_OFFSET + (room_no as usize * ROOM_SIZE);

        let mut cells: Vec<JswRawCell> = vec![];

        // Read conveyor direction
        let mut conveyor_direction = ConveyorDirection::Right;
        data.set_rpos(room_offset + 0x26f);
        if data.read_u8()? > 0 {
            conveyor_direction = ConveyorDirection::Left;
        }

        for i in 0..CELL_COUNT {
            data.set_rpos(room_offset + 0x220 + (i * CELL_LENGTH));
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

            let behaviour = Self::get_cell_behaviour(i, conveyor_direction);

            let cell = JswRawCell::new(i as u8, attribute, behaviour, sprite);
            cells.push(cell);
        }

        Ok(cells)
    }

    fn get_cell_behaviour(cell_no: usize, conveyor_direction: ConveyorDirection) -> CellBehaviour {
        match cell_no {
            0 => CellBehaviour::Air,
            1 => CellBehaviour::Water,
            2 => CellBehaviour::Crumbly,
            3 => CellBehaviour::Earth,
            4 => {
                if conveyor_direction == ConveyorDirection::Left {
                    CellBehaviour::LConveyor
                } else {
                    CellBehaviour::RConveyor
                }
            }
            5 => CellBehaviour::Fire,
            6 => CellBehaviour::Fire,
            7 => CellBehaviour::Water,
            _ => CellBehaviour::Air,
        }
    }
}

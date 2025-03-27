use std::{cell, io};

use bytebuffer::ByteBuffer;
use macroquad::text;

use super::RawParser;
use crate::{
    Result,
    game::GameType,
    raw_game::{CellBehaviour, JswRawCell, JswRawGame, JswRawRoom, ROOM_LAYOUT_SIZE},
};

const RAM_OFFSET: usize = 0x5C00;
const TEXT_COMPRESSION_TABLE_OFFSET: usize = 0xFA81 - RAM_OFFSET;
const ROOMS_OFFSET: usize = 0x0B000 - RAM_OFFSET;
const ROOM_TABLE_POINTER_ADDR: usize = 0x7E69 - RAM_OFFSET;
const ROOM_SIZE: usize = 0x400;
const ROOM_COUNT: u8 = 134;
const ROOM_NAME_LENGTH: usize = 0x20;

pub struct RawJsw2Game {
    //
}

impl RawParser for RawJsw2Game {
    fn extract_game(game_type: GameType, data: &mut ByteBuffer) -> Result<JswRawGame> {
        // // Extract the word dictionary
        // let dictionary = Self::extract_dictionary(data)?;

        let raw_game = JswRawGame {
            game_type,
            rooms: Self::extract_rooms(data)?,
        };

        Ok(raw_game)
    }
}

impl RawJsw2Game {
    // fn extract_dictionary(data: &mut ByteBuffer) -> Result<Vec<String>> {
    //     let mut dictionary: Vec<String> = vec![];

    //     data.set_rpos(DICTIONARY_OFFSET);

    //     let byte = data.read_u8()?;
    //     if byte != 0x80 {
    //         return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data").into());
    //     }

    //     let mut raw_word = vec![];
    //     loop {
    //         // Check not at end of buffer
    //         if data.get_rpos() >= data.len() {
    //             break;
    //         }

    //         let mut byte = data.read_u8()?;
    //         if byte & 0x80 != 0 {
    //             byte &= 0x7F;
    //             if (0x20..=0x7E).contains(&byte) {
    //                 raw_word.push(byte);

    //                 // Convert the bytes to a string
    //                 let word = String::from_utf8(raw_word.clone())
    //                     .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

    //                 dictionary.push(word);

    //                 raw_word.clear();
    //             } else {
    //                 break;
    //             }
    //         } else if (0x20..=0x7E).contains(&byte) {
    //             raw_word.push(byte);
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(dictionary)
    // }

    fn extract_rooms(data: &mut ByteBuffer) -> Result<Vec<JswRawRoom>> {
        let mut rooms: Vec<JswRawRoom> = vec![];

        // TODO - calculate the room count (ROOM0 - TABLE) / 2

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
        // Get the address of the room table
        data.set_rpos(ROOM_TABLE_POINTER_ADDR);
        let room_table_addr = Self::read_addr_16(data)? as usize;

        // Get the address of the room
        let room_addr_addr = room_table_addr + (room_no as usize * 2);
        data.set_rpos(room_addr_addr);
        let room_offset = Self::read_addr_16(data)? as usize;
        data.set_rpos(room_offset);

        // Room name
        data.set_rpos(room_offset + 0xC);
        let name = Self::read_text(data)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to read text"))?;

        // let raw_name = read_string(data, ROOM_NAME_LENGTH)?;
        // let name = raw_name.trim().to_string();

        // Cells
        let cells = Self::extract_cells(data, room_no, room_offset)?;

        // Layout
        let layout = Self::extract_room_layout(data, room_no, room_offset, &cells)?;

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
        _room_no: u8,
        room_offset: usize,
        _cells: &[JswRawCell],
    ) -> Result<[u8; ROOM_LAYOUT_SIZE]> {
        // Get the address of the room layout
        data.set_rpos(room_offset);
        let room_layout_addr = Self::read_addr_16(data)? as usize;
        data.set_rpos(room_layout_addr);

        Ok([0; ROOM_LAYOUT_SIZE])
    }

    fn extract_cells(
        data: &mut ByteBuffer,
        _room_no: u8,
        room_offset: usize,
    ) -> Result<Vec<JswRawCell>> {
        let mut cells: Vec<JswRawCell> = vec![];

        data.set_rpos(room_offset + 2); // hbits offset
        let hbits = data.read_u8()?;

        for (i, cell_low_byte) in data.read_bytes(8)?.iter().enumerate() {
            let cell_word = u16::from_be_bytes([(hbits >> (7 - i)) & 0x01, *cell_low_byte]);
            let cell_addr = ((cell_word as usize) * 9 + 0x8C78) - RAM_OFFSET;

            data.set_rpos(cell_addr);
            let attribute = data.read_u8()?;

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

            let behaviour = Self::get_cell_behaviour(i);

            let cell = JswRawCell::new(i as u8, attribute, behaviour, sprite);
            cells.push(cell);
        }

        Ok(cells)
    }

    // Read text from the current position
    fn read_text(data: &mut ByteBuffer) -> Result<String> {
        let mut length = 0;

        let mut read_text = |data: &mut ByteBuffer, text_offset: usize| -> Result<String> {
            let mut t = String::new();
            let mut offset = text_offset;

            loop {
                data.set_rpos(offset);
                let mut byte = data.read_u8()?;
                offset += 1;

                let is_last_byte = (byte & 0x80) == 0x80;
                byte &= 0x7f;

                if !is_last_byte && byte <= 31 {
                    let s = Self::read_compressed_text(data, byte as usize)?;
                    // Append s to t
                    if !s.is_empty() {
                        t.push_str(&s);
                        t.push(' ');
                    }
                } else if (0x20..=0x7E).contains(&byte) {
                    t.push(byte as char);
                } else {
                    // invalid character
                }

                // Increment the length
                length += 1;

                if is_last_byte {
                    break;
                }
            }

            Ok(t.trim_end().into())
        };

        let offset = data.get_rpos();
        read_text(data, offset)
    }

    /// Read text from the compressed text table
    fn read_compressed_text(data: &mut ByteBuffer, word_no: usize) -> Result<String> {
        let mut length = 0;

        let mut read_text = |data: &mut ByteBuffer, text_offset: usize| -> Result<String> {
            let mut t = String::new();
            let mut offset = text_offset;

            loop {
                data.set_rpos(offset);
                let mut byte = data.read_u8()?;
                offset += 1;

                let is_last_byte = (byte & 0x80) == 0x80;
                byte &= 0x7f;

                if !is_last_byte && byte <= 31 {
                    let s = Self::read_compressed_text(data, byte as usize)?;
                    // Append s to t
                    if !s.is_empty() {
                        t.push_str(&s);
                        t.push(' ');
                    }
                } else if (0x20..=0x7E).contains(&byte) {
                    t.push(byte as char);
                } else {
                    // invalid character
                }

                // Increment the length
                length += 1;

                if is_last_byte {
                    break;
                }
            }

            Ok(t)
        };

        // Find the offset of the word in the text compression table
        let mut word_index = 0;
        let mut word_start_offset = TEXT_COMPRESSION_TABLE_OFFSET;
        data.set_rpos(word_start_offset);

        loop {
            let byte = data.read_u8()?;
            word_start_offset += 1;

            let is_last_byte = (byte & 0x80) == 0x80;
            if is_last_byte {
                word_index += 1;
            }

            if word_no == word_index {
                break;
            }
        }

        read_text(data, word_start_offset)
    }

    fn read_addr_16(data: &mut ByteBuffer) -> Result<u16> {
        let value = data.read_u16()? - RAM_OFFSET as u16;
        Ok(value)
    }

    fn get_cell_behaviour(cell_no: usize) -> CellBehaviour {
        match cell_no {
            0 => CellBehaviour::Water,
            1 => CellBehaviour::Earth,
            2 => CellBehaviour::Fire,
            3 => CellBehaviour::RRamp,
            4 => CellBehaviour::LConveyor,
            5 => CellBehaviour::Item,
            6 => CellBehaviour::RRamp,
            7 => CellBehaviour::RConveyor,
            _ => CellBehaviour::Air,
        }
    }
}

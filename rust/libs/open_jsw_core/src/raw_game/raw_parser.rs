use std::io;

use bytebuffer::ByteBuffer;

use super::{JswRawCell, JswRawGame, JswRawRoom};
use crate::{Result, game::GameType};

pub mod jsw2_parser;
pub mod jsw_parser;
pub mod mm_parser;

pub const RAM_OFFSET: usize = 0x8000;

pub trait RawParser {
    fn extract_game(game_type: GameType, data: &mut ByteBuffer) -> Result<JswRawGame> {
        let raw_game = JswRawGame {
            game_type,
            rooms: Self::extract_rooms(data)?,
        };

        Ok(raw_game)
    }

    fn extract_rooms(data: &mut ByteBuffer) -> Result<Vec<JswRawRoom>>;
    fn extract_room(data: &mut ByteBuffer, room_no: u8) -> Result<JswRawRoom>;
    fn extract_room_layout(
        data: &mut ByteBuffer,
        room_no: u8,
        cells: &Vec<JswRawCell>,
    ) -> Result<[u8; 512]>;
    fn extract_cells(data: &mut ByteBuffer, room_no: u8) -> Result<Vec<JswRawCell>>;
}

pub fn read_string(data: &mut ByteBuffer, length: usize) -> io::Result<String> {
    let s = String::from_utf8(data.read_bytes(length)?)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

    Ok(s)
}

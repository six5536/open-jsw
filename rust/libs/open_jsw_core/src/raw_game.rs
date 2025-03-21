// use byteorder::{LittleEndian, ReadBytesExt};
use std::{fs::File, io::Read, path::PathBuf};

use bytebuffer::{ByteBuffer, Endian::BigEndian};
use raw_game_identifier::RawGameData;
use raw_parser::{
    RawParser, jsw_parser::RawJswGame, jsw2_parser::RawJsw2Game, mm_parser::RawMmGame,
};

use crate::{Result, game::GameType};

mod raw_game_identifier;
mod raw_parser;

pub const ROOM_LAYOUT_WIDTH: usize = 32;
pub const ROOM_LAYOUT_HEIGHT: usize = 16;
pub const ROOM_LAYOUT_SIZE: usize = ROOM_LAYOUT_WIDTH * ROOM_LAYOUT_HEIGHT;

pub struct JswRawGame {
    pub game_type: GameType,
    pub rooms: Vec<JswRawRoom>,
}

pub struct JswRawRoom {
    pub room_no: u8,
    pub name: String,
    pub layout: [u8; ROOM_LAYOUT_SIZE],
}

impl JswRawGame {
    pub fn new(game_type: GameType, rooms: Vec<JswRawRoom>) -> Self {
        Self { game_type, rooms }
    }

    #[allow(dead_code)]
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let file = File::open(path)?;

        Self::from_reader(file)
    }

    pub fn from_reader(mut rdr: impl Read) -> Result<Self> {
        let bytes = &mut vec![];
        rdr.read_to_end(bytes)?;

        Self::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let game = raw_game_identifier::identify_game(bytes)?;

        Self::game_buffer_to_game(game)
    }

    fn game_buffer_to_game(game_data: RawGameData) -> Result<Self> {
        let game_bytes = game_data.game_bytes();

        let mut data = ByteBuffer::from_bytes(game_bytes);
        data.set_endian(BigEndian);

        let game_type = game_data.game_type().clone();

        match game_type {
            GameType::MM => RawMmGame::extract_game(game_type, &mut data),
            GameType::JSW => RawJswGame::extract_game(game_type, &mut data),
            GameType::JSW2 => RawJsw2Game::extract_game(game_type, &mut data),
        }
    }
}

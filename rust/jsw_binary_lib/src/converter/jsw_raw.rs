// use byteorder::{LittleEndian, ReadBytesExt};
use bytebuffer::ByteBuffer;
use bytebuffer::Endian::BigEndian;
use std::{
    fs::File,
    io::{self, Read},
};

use std::path::PathBuf;

use super::{
    jsw_raw_parser::JswGame,
    jsw_signatures::{self, Game, GameType},
    jsw2_raw_parser::Jsw2Game,
    mm_raw_parser::MmGame,
};

pub struct JswRaw {
    pub rooms: Vec<JswRawRoom>,
}

pub struct JswRawRoom {
    pub room_no: u8,
    pub name: String,
    // pub item3: i32,
}

pub fn from_file(path: &PathBuf) -> io::Result<JswRaw> {
    let bytes = &mut vec![];
    let game = JswRawReader::from_file(path, bytes)?;

    game_buffer_to_game(game)
}

pub fn from_reader(rdr: impl Read) -> io::Result<JswRaw> {
    let bytes = &mut vec![];
    let game = JswRawReader::from_reader(rdr, bytes)?;

    game_buffer_to_game(game)
}

pub fn from_bytes<'a>(bytes: &'a [u8]) -> io::Result<JswRaw> {
    let game = JswRawReader::from_bytes(bytes)?;

    game_buffer_to_game(game)
}

fn game_buffer_to_game<'a>(game: Game<'a>) -> io::Result<JswRaw> {
    let game_bytes = game.game_bytes();

    let mut data = ByteBuffer::from_bytes(game_bytes);
    data.set_endian(BigEndian);

    match game.game_type() {
        GameType::MM => {
            // gameParser = MmGame();
            MmGame::extract_game(&mut data)
        }
        GameType::JSW => {
            // gameParser = JswGame();
            JswGame::extract_game(&mut data)
        }
        GameType::JSW2 => {
            // gameParser = Jsw2Game();
            Jsw2Game::extract_game(&mut data)
        }
    }
}

struct JswRawReader {}

impl<'a> JswRawReader {
    fn from_file(path: &PathBuf, bytes: &'a mut Vec<u8>) -> io::Result<Game<'a>> {
        let file = File::open(path)?;

        Self::from_reader(file, bytes)
    }

    fn from_reader(mut rdr: impl Read, bytes: &'a mut Vec<u8>) -> io::Result<Game<'a>> {
        rdr.read_to_end(bytes)?;

        Self::from_bytes(bytes)
    }

    fn from_bytes(bytes: &'a [u8]) -> io::Result<Game<'a>> {
        // Identify the game type
        let game = jsw_signatures::identify(bytes)?;

        Ok(game)
    }
}

pub trait JswRawParser {
    fn extract_game(data: &mut ByteBuffer) -> io::Result<JswRaw> {
        let raw_game = JswRaw {
            rooms: Self::extract_rooms(data)?,
        };

        Ok(raw_game)
    }

    fn extract_rooms(data: &mut ByteBuffer) -> io::Result<Vec<JswRawRoom>>;
    fn extract_room(data: &mut ByteBuffer, room_no: u8) -> io::Result<JswRawRoom>;
}

use std::io;

#[derive(Debug, Clone)]
pub enum GameType {
    MM,
    JSW,
    JSW2,
}

pub fn identify<'a>(bytes: &'a [u8]) -> Result<Game<'a>, std::io::Error> {
    let games = vec![
        Game::new(
            GameType::MM,
            MM_SIGNATURE,
            MM_SIGNATURE_OFFSET,
            MM_GAME_LENGTH,
            bytes,
        ),
        Game::new(
            GameType::JSW,
            JSW_SIGNATURE,
            JSW_SIGNATURE_OFFSET,
            JSW_GAME_LENGTH,
            bytes,
        ),
        Game::new(
            GameType::JSW2,
            JSW2_SIGNATURE,
            JSW2_SIGNATURE_OFFSET,
            JSW2_GAME_LENGTH,
            bytes,
        ),
    ];

    for mut game in games {
        let valid = game.identify();
        if valid {
            let game_type = game.game_type();
            println!("Found game type: {:?}", game_type);
            return Ok(game);
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "Game not recognised"))
}

pub struct Game<'a> {
    game_type: GameType,
    valid: bool,

    // Game data
    signature: &'static [u8],
    signature_offset: usize,
    data_length: usize,
    bytes: &'a [u8],
    start_index: usize,
    // Game constants
}

impl<'a> Game<'a> {
    fn new(
        game_type: GameType,
        signature: &'static [u8],
        signature_offset: usize,
        data_length: usize,
        bytes: &'a [u8],
    ) -> Self {
        Self {
            game_type,
            signature,
            signature_offset,
            data_length,
            bytes,
            start_index: 0,
            valid: false,
        }
    }
    pub fn game_type(&self) -> &GameType {
        &self.game_type
    }

    fn identify(&mut self) -> bool {
        let signature = self.signature;
        let mut signature_index = 0;
        for (i, byte) in self.bytes.iter().enumerate() {
            if *byte == signature[signature_index] {
                signature_index += 1;
                if signature_index == signature.len() {
                    let start_offset = self.signature_offset + signature.len() - 1;
                    if i < start_offset {
                        signature_index = 0;
                        continue;
                    }
                    self.start_index = i - start_offset;
                    self.valid = self.bytes.len() >= self.start_index + self.data_length;
                    return self.valid;
                }
            } else {
                signature_index = 0;
            }
        }
        false
    }

    fn game_length(&self) -> usize {
        self.start_index + self.data_length
    }

    pub fn game_bytes(&self) -> &'a [u8] {
        &self.bytes[self.start_index..self.game_length()]
    }
}

const RAM_OFFSET: usize = 0x8000;

const MM_SIGNATURE: &[u8] = &[
    0x06, 0x10, 0xCB, 0x41, 0x1A, 0x28, 0x04, 0xA6, 0xC0, 0x1A, 0xB6, 0x77, 0x2C, 0x13, 0xCB, 0x41,
    0x1A, 0x28, 0x04, 0xA6, 0xC0, 0x1A, 0xB6, 0x77, 0x2D, 0x24, 0x13, 0x7C, 0xE6, 0x07, 0x20, 0x10,
    0x7C, 0xD6, 0x08, 0x67, 0x7D, 0xC6, 0x20, 0x6F, 0xE6, 0xE0, 0x20, 0x04, 0x7C, 0xC6, 0x08, 0x67,
    0x10, 0xD0, 0xAF, 0xC9, 0x3A, 0x07,
];
const MM_SIGNATURE_OFFSET: usize = 0x0FF4;
const MM_GAME_LENGTH: usize = 0x7FFF; // 32767;

const JSW_SIGNATURE: &[u8] = &[
    0x06, 0x10, 0xCB, 0x41, 0x1A, 0x28, 0x04, 0xA6, 0xC0, 0x1A, 0xB6, 0x77, 0x2C, 0x13, 0xCB, 0x41,
    0x1A, 0x28, 0x04, 0xA6, 0xC0, 0x1A, 0xB6, 0x77, 0x2D, 0x24, 0x13, 0x7C, 0xE6, 0x07, 0x20, 0x10,
    0x7C, 0xD6, 0x08, 0x67, 0x7D, 0xC6, 0x20, 0x6F, 0xE6, 0xE0, 0x20, 0x04, 0x7C, 0xC6, 0x08, 0x67,
    0x10, 0xD0, 0xAF, 0xC9, 0x3A, 0xE9,
];
const JSW_SIGNATURE_OFFSET: usize = 0x1456;
const JSW_GAME_LENGTH: usize = 0x7CFF;

const JSW2_SIGNATURE: &[u8] = &[
    0x09, 0x10, 0xCB, 0x41, 0x1A, 0x28, 0x04, 0xA6, 0xC0, 0x1A, 0xB6, 0x77, 0x2C, 0x13, 0xCB, 0x41,
    0x1A, 0x28, 0x04, 0xA6, 0xC0, 0x1A, 0xB6, 0x99, 0x2D, 0x24, 0x13, 0x7C, 0xE6, 0x07, 0x20, 0x10,
    0x7C, 0xD6, 0x08, 0x67, 0x7D, 0xC6, 0x20, 0x6F, 0x99, 0xE0, 0x20, 0x04, 0x7C, 0xC6, 0x08, 0x67,
    0x10, 0xD0, 0xAF, 0xC9, 0x3A, 0xE9,
];
const JSW2_SIGNATURE_OFFSET: usize = 0;
const JSW2_GAME_LENGTH: usize = 0x8FF4;

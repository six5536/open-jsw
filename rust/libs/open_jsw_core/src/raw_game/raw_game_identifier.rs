/// Identifies the game type from the game data.
///
/// The bin (memory dump), tap and tzx files all contain the game in the same format for MM and JSW.
/// For JSW2, the tap and tzx are compressed / encrypted and need to be detected and decompressed (TODO!).
/// The game data is identified by a signature that is unique to each game.
///
/// The offset from the start of the game data to the signature is used to find the start of the game data.
///
use crate::{Error, Result, game::GameType};

pub fn identify_game(bytes: &[u8]) -> Result<RawGameData> {
    let games = vec![
        RawGameData::new(
            GameType::MM,
            MM_SIGNATURE,
            MM_SIGNATURE_OFFSET,
            MM_GAME_LENGTH,
            bytes,
        ),
        RawGameData::new(
            GameType::JSW,
            JSW_SIGNATURE,
            JSW_SIGNATURE_OFFSET,
            JSW_GAME_LENGTH,
            bytes,
        ),
        RawGameData::new(
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
    // Err(io::Error::new(io::ErrorKind::Other, "Game not recognised").into())
    Err(Error::GameNotRecognised)
}

pub struct RawGameData<'a> {
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

impl<'a> RawGameData<'a> {
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
        let bytes_len = self.bytes.len();

        for (i, _byte) in self.bytes.iter().enumerate() {
            // Check for end of bytes
            if i + signature.len() > bytes_len {
                break;
            }

            // Get a window (slice) of bytes the same length as the signature
            let possible_signature = &self.bytes[i..i + signature.len()];

            // Compare the window of bytes with the signature
            if possible_signature == signature {
                if i < self.signature_offset {
                    // Cannot be a valid signature if the signature offset is not reached
                    continue;
                }
                self.start_index = i - self.signature_offset;
                self.valid = bytes_len >= self.start_index + self.data_length;
                return self.valid;
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

const MM_SIGNATURE: &[u8] = &[
    0x06, 0x10, 0xCB, 0x41, 0x1A, 0x28, 0x04, 0xA6, 0xC0, 0x1A, 0xB6, 0x77, 0x2C, 0x13, 0xCB, 0x41,
    0x1A, 0x28, 0x04, 0xA6, 0xC0, 0x1A, 0xB6, 0x77, 0x2D, 0x24, 0x13, 0x7C, 0xE6, 0x07, 0x20, 0x10,
    0x7C, 0xD6, 0x08, 0x67, 0x7D, 0xC6, 0x20, 0x6F, 0xE6, 0xE0, 0x20, 0x04, 0x7C, 0xC6, 0x08, 0x67,
    0x10, 0xD0, 0xAF, 0xC9, 0x3A, 0x07,
];
const MM_SIGNATURE_OFFSET: usize = 0x0FF4;
const MM_GAME_LENGTH: usize = 0x7FFF; // 32767; 32kB // TODO - check this length - seems to be 1 byte too short (the bin file)

const JSW_SIGNATURE: &[u8] = &[
    0x06, 0x10, 0xCB, 0x41, 0x1A, 0x28, 0x04, 0xA6, 0xC0, 0x1A, 0xB6, 0x77, 0x2C, 0x13, 0xCB, 0x41,
    0x1A, 0x28, 0x04, 0xA6, 0xC0, 0x1A, 0xB6, 0x77, 0x2D, 0x24, 0x13, 0x7C, 0xE6, 0x07, 0x20, 0x10,
    0x7C, 0xD6, 0x08, 0x67, 0x7D, 0xC6, 0x20, 0x6F, 0xE6, 0xE0, 0x20, 0x04, 0x7C, 0xC6, 0x08, 0x67,
    0x10, 0xD0, 0xAF, 0xC9, 0x3A, 0xE9,
];
const JSW_SIGNATURE_OFFSET: usize = 0x1456;
// const JSW_GAME_LENGTH: usize = 0x7CFF;
const JSW_GAME_LENGTH: usize = 0x7FFF; // TODO - check this length (the bin file)

const JSW2_SIGNATURE: &[u8] = &[
    0x00, 0x70, 0x00, 0x68, 0x01, 0x61, 0xD6, 0x81, 0xD6, 0xD0, 0x00, 0x1C, 0x07, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x50, 0xC0, 0x51, 0xC0, 0x52, 0xC0,
    0x53, 0xC0, 0x54, 0xC0, 0x55, 0xC0, 0x56, 0xC0, 0x57, 0xE0, 0x50, 0xE0, 0x51, 0xE0, 0x52, 0xE0,
    0x53, 0xE0, 0x54, 0xE0, 0x55, 0xE0,
];

const JSW2_SIGNATURE_OFFSET: usize = 0x1470;
// const JSW2_GAME_LENGTH: usize = 0x7FFF;
const JSW2_GAME_LENGTH: usize = 0xA3FF; // TODO - check this length

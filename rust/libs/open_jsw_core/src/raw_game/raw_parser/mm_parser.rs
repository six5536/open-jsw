use bytebuffer::ByteBuffer;

use super::{RAM_OFFSET, RawParser, read_string};
use crate::{Result, raw_game::JswRawRoom};

const ROOMS_OFFSET: usize = 0x0B000 - RAM_OFFSET;
const ROOM_SIZE: usize = 0x400;
const ROOM_COUNT: u8 = 20;
const ROOM_NAME_LENGTH: usize = 0x20;

pub struct RawMmGame {
    //
}

impl RawParser for RawMmGame {
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

        let room = JswRawRoom { room_no, name };

        Ok(room)
    }
}

// use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    fs::File,
    io::{self, Read},
};

use bytebuffer::ByteBuffer;
use bytebuffer::Endian::BigEndian;

const ROOMS_OFFSET: usize = 0xb000;
const ROOM_SIZE: usize = 0x400;
const ROOM_COUNT: u8 = 20;
const ROOM_NAME_LENGTH: usize = 0x20;

pub struct JswRaw {
    pub rooms: Vec<JswRawRoom>,
}

pub struct JswRawRoom {
    pub room_no: u8,
    pub name: String,
    // pub item3: i32,
}

impl JswRaw {
    pub fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let bytes = &mut vec![];
        rdr.read_to_end(bytes)?;

        Self::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        let mut data = ByteBuffer::from_bytes(bytes);
        data.set_endian(BigEndian);

        let mut rooms = vec![];

        // TODO - work out the file format
        let mut room_no: u8 = 0;
        while room_no < ROOM_COUNT {
            Self::extract_room(&mut data, room_no, &mut rooms)?;

            room_no += 1;
        }

        // let roomNo = data.read_u8()?;
        // // let item2 = rdr.read_u16::<LittleEndian>()?;
        // let item2 = u16::from_be(rdr.read_u16());
        // // let item3 = rdr.read_i32::<LittleEndian>()?;

        Ok(JswRaw { rooms })
    }

    fn extract_room(
        data: &mut ByteBuffer,
        room_no: u8,
        rooms: &mut Vec<JswRawRoom>,
    ) -> io::Result<()> {
        let room_offset = ROOMS_OFFSET + (room_no as usize * ROOM_SIZE);
        data.set_rpos(room_offset);

        // Room name
        data.set_rpos(room_offset + 0x200);
        let name = Self::read_string(data, ROOM_NAME_LENGTH)?;

        let room = JswRawRoom { room_no, name };
        rooms.push(room);

        Ok(())
    }

    fn read_string(data: &mut ByteBuffer, length: usize) -> io::Result<String> {
        let s = String::from_utf8(data.read_bytes(length)?)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        Ok(s)
    }
}

// pub fn convert(mut rdr: impl Read) -> io::Result<Configuration> {
//     println!("Converting...");

//     let file = File::open("/dev/random").unwrap();

//     let config = Configuration::from_reader(file);
// }

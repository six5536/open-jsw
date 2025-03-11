use std::io;

use bytebuffer::ByteBuffer;

pub const RAM_OFFSET: usize = 0x8000;

pub fn read_string(data: &mut ByteBuffer, length: usize) -> io::Result<String> {
    let s = String::from_utf8(data.read_bytes(length)?)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

    Ok(s)
}

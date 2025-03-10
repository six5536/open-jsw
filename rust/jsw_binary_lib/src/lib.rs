use converter::jsw_raw::JswRaw;
use std::{
    fmt::Error,
    io::{self, ErrorKind, Read},
};

mod converter;

pub fn convert(rdr: impl Read) -> io::Result<JswRaw> {
    JswRaw::from_reader(rdr)
    // Err(io::Error::new(io::ErrorKind::Other, "your message here"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}

// use derive_more::From;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;
// pub type Error = Box<dyn std::error::Error>; // For early dev

#[derive(Error, Debug)]
pub enum Error {
    // Custom errors (for early dev)
    #[error("{}", .0)]
    Custom(String),

    // Module errors
    #[error("{:?} {}", message, num)]
    TestError { message: String, num: i64 },

    // External errors
    #[error("OpenJswCore: {}", .0)]
    OpenJswCore(#[from] open_jsw_core::Error),

    #[error("OpenJswTiled: {}", .0)]
    OpenJswTiled(#[from] open_jsw_tiled::Error),

    #[error("IO::{:?}: {}", .0, .0)]
    Io(#[from] std::io::Error),

    #[error("FlexiLogger::{:?}: {}", .0, .0)]
    FlexiLogger(#[from] flexi_logger::FlexiLoggerError),
}

// impl Error {
//     //
// }

// region:    --- Error Boilerplate

// impl core::fmt::Display for Error {
//     fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
//         write!(fmt, "{self:?}")
//     }
// }

// impl std::error::Error for Error {}

// endregion: --- Error Boilerplate

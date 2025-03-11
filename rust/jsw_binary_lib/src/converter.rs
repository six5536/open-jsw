use thiserror::Error;

mod converter_utils;
mod jsw2_raw_parser;
pub mod jsw_raw;
mod jsw_raw_parser;
pub mod jsw_signatures;
mod mm_raw_parser;

#[derive(Error, Debug)]
pub enum ConverterError {
    #[error("File format not recognised '{src:?}'")]
    UnrecognisedFormat { src: String },
    #[error("Failed to read '{src:?}' at '{location:?}: {data:?}'")]
    ReadError {
        src: String,
        location: String,
        data: String,
    },
    #[error("Unknown conversion error '{src:?}'")]
    Unknown { src: String },
}

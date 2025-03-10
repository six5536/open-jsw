use thiserror::Error;

pub mod jsw_raw;

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

// use thiserror::Error;

mod converter_utils;
pub mod raw_parsers;

pub mod jsw_raw;

pub mod jsw_signatures;

// #[derive(Error, Debug)]
// pub enum ConverterError {
//     #[error("File format not recognised '{src:?}'")]
//     UnrecognisedFormat { src: String },
//     #[error("Failed to read '{src:?}' at '{location:?}: {data:?}'")]
//     ReadError {
//         src: String,
//         location: String,
//         data: String,
//     },
//     #[error("Unknown conversion error '{src:?}'")]
//     Unknown { src: String },
// }

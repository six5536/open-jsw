use nanoserde::DeJsonErrReason;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;
// pub type Error = Box<dyn std::error::Error>; // For early dev

#[derive(Error, Debug)]
pub enum Error {
    #[error("[Line:{}, Col:{}] {:?}", .line, .col, .msg)]
    DeJsonErr {
        msg: DeJsonErrReason,
        line: usize,
        col: usize,
    },
    #[error("{}", .layer)]
    NonUniqueLayerName { layer: String },
    #[error("{}", .texture)]
    TextureNotFound { texture: String },
    #[error("{}", .layer_type)]
    LayerTypeNotFound { layer_type: String },
}

impl From<nanoserde::DeJsonErr> for Error {
    fn from(error: nanoserde::DeJsonErr) -> Error {
        Error::DeJsonErr {
            msg: error.msg.clone(),
            line: error.line,
            col: error.col,
        }
    }
}

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Error::DeJsonErr { .. } | Error::TextureNotFound { .. } => {
//                 std::fmt::Debug::fmt(self, f)
//             }
//             Error::NonUniqueLayerName { layer } => write!(
//                 f,
//                 "Layer name should be unique to load tiled level in macroquad, non-unique layer name: {}",
//                 layer
//             ),
//             Error::LayerTypeNotFound { layer_type } => {
//                 write!(f, "{} type layer not found.", layer_type)
//             }
//         }
//     }
// }

// impl std::error::Error for Error {}

use nanoserde::DeJson;
use tiled::map::Map;

// Import and re-export the `error` module
pub use self::error::{Error, Result};
mod error;

pub mod tiled;

// TODO: Differentiate using 'type' when deserializing, so that the types are better defined.
pub fn load_map(data: &str) -> Result<Map> {
    let map: Map = DeJson::deserialize_json(data)?;

    Ok(map)
}

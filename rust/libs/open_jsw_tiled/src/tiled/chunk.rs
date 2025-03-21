use nanoserde::{DeJson, SerJson};

/// Represents a Layer in the map.
#[derive(Clone, Debug, Default, DeJson, SerJson)]
#[nserde(default)]
pub struct Chunk {
    /// Array of unsigned int (GIDs) or base64-encoded data.
    pub data: Vec<u32>,

    /// Height in tiles.
    pub height: u32,

    /// Width in tiles.
    pub width: u32,

    /// X coordinate in tiles.
    pub x: i32,

    /// Y coordinate in tiles.
    pub y: i32,
}

use nanoserde::DeJson;

/// Represents a Layer in the map.
#[derive(Clone, Debug, Default, DeJson)]
#[nserde(default)]
pub struct Chunk {
    /// Array of unsigned int (GIDs) or base64-encoded data.
    pub data: Vec<u32>,

    /// Height in tiles.
    pub height: i32,

    /// Width in tiles.
    pub width: i32,

    /// X coordinate in tiles.
    pub x: i32,

    /// Y coordinate in tiles.
    pub y: i32,
}

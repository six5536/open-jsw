use nanoserde::{DeJson, SerJson};

/// Represents a Point in Tiled.
#[derive(Clone, Debug, Default, DeJson, SerJson)]
#[nserde(default)]
pub struct Point {
    /// X coordinate in pixels.
    pub x: i32,

    /// Y coordinate in pixels.
    pub y: i32,
}

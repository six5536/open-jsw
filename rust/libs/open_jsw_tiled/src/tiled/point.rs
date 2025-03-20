use nanoserde::DeJson;

/// Represents a Point in Tiled.
#[derive(Clone, Debug, Default, DeJson)]
#[nserde(default)]
pub struct Point {
    /// X coordinate in pixels.
    pub x: f64,

    /// Y coordinate in pixels.
    pub y: f64,
}

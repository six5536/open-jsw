#![allow(clippy::question_mark)]

use nanoserde::DeJson;

use super::{point::Point, property::Property, text::Text};

/// Represents a Layer in the map.
#[derive(Clone, Debug, Default, DeJson)]
#[nserde(default)]
pub struct Object {
    /// Used to mark an object as an ellipse.
    pub ellipse: Option<bool>,

    /// Global tile ID, only if the object represents a tile.
    pub gid: Option<i32>,

    /// Height in pixels.
    pub height: f64,

    /// Incremental ID, unique across all objects.
    pub id: i32,

    /// Name assigned to the object in the editor.
    pub name: String,

    /// Used to mark an object as a point.
    pub point: Option<bool>,

    /// Array of points, if the object is a polygon.
    pub polygon: Option<Vec<Point>>,

    /// Array of points, if the object is a polyline.
    pub polyline: Option<Vec<Point>>,

    /// Array of properties (optional).
    pub properties: Option<Vec<Property>>,

    /// Angle in degrees clockwise.
    pub rotation: f64,

    /// Reference to a template file, in case the object is a template instance.
    pub template: Option<String>,

    /// Text object details (only used for text objects).
    pub text: Option<Text>,

    /// The class of the object (was saved as `class` in 1.9, optional).
    #[nserde(rename = "type")]
    #[nserde(rename = "class")]
    pub typ: Option<String>,

    /// Whether the object is shown in the editor.
    pub visible: bool,

    /// Width in pixels.
    pub width: f64,

    /// X coordinate in pixels.
    pub x: f64,

    /// Y coordinate in pixels.
    pub y: f64,
}

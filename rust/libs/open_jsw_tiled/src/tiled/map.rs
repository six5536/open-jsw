#![allow(clippy::question_mark)]

use nanoserde::{DeJson, SerJson};

use super::{MAP_VERSION, TILED_VERSION, layer::Layer, property::Property, tileset::Tileset};

const DEFAULT_MINUS_ONE_I32: i32 = -1;

/// https://doc.mapeditor.org/en/stable/reference/json-map-format/#map
/// Represents a Map in the Tiled map editor.
#[derive(Clone, Debug, Default, DeJson, SerJson)]
#[nserde(default)]
pub struct Map {
    /// Hex-formatted color (#RRGGBB or #AARRGGBB) (optional).
    pub backgroundcolor: Option<String>,

    /// The class of the map (since 1.9, optional).
    pub class: Option<String>,

    /// The compression level to use for tile layer data (defaults to -1, which means to use the algorithm default).
    #[nserde(default = "DEFAULT_MINUS_ONE_I32")]
    pub compressionlevel: i32,

    /// Number of tile rows.
    pub height: u32,

    /// Length of the side of a hex tile in pixels (hexagonal maps only).
    pub hexsidelength: Option<i32>,

    /// Whether the map has infinite dimensions.
    pub infinite: bool,

    /// Array of layers.
    pub layers: Vec<Layer>,

    /// Auto-increments for each layer.
    pub nextlayerid: u32,

    /// Auto-increments for each placed object.
    pub nextobjectid: u32,

    /// Map orientation: "orthogonal", "isometric", "staggered", or "hexagonal".
    pub orientation: MapOrientation,

    /// X coordinate of the parallax origin in pixels (since 1.8, default: 0).
    pub parallaxoriginx: f64,

    /// Y coordinate of the parallax origin in pixels (since 1.8, default: 0).
    pub parallaxoriginy: f64,

    /// Array of properties (optional).
    pub properties: Option<Vec<Property>>,

    /// Render order: "right-down" (default), "right-up", "left-down", or "left-up".
    /// Currently only supported for orthogonal maps.
    pub renderorder: RenderOrder,

    /// Stagger axis: "x" or "y" (for staggered/hexagonal maps only).
    pub staggeraxis: Option<Axis>,

    /// Stagger index: "odd" or "even" (for staggered/hexagonal maps only).
    pub staggerindex: Option<Parity>,

    /// The Tiled version used to save the file.
    pub tiledversion: String,

    /// Map grid tile height.
    pub tileheight: u32,

    /// Array of tilesets.
    pub tilesets: Vec<Tileset>,

    /// Map grid tile width.
    pub tilewidth: u32,

    /// Type of the map (always "map" since 1.0).
    #[nserde(rename = "type")]
    pub typ: MapType,

    /// The JSON format version (previously a number, saved as string since 1.6).
    pub version: String,

    /// Number of tile columns.
    pub width: u32,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub enum MapType {
    #[default]
    #[nserde(rename = "map")]
    Map,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub enum MapOrientation {
    #[default]
    #[nserde(rename = "orthogonal")]
    Orthogonal,
    #[nserde(rename = "isometric")]
    Isometric,
    #[nserde(rename = "staggered")]
    Staggered,
    #[nserde(rename = "hexagonal")]
    Hexagonal,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub enum RenderOrder {
    #[default]
    #[nserde(rename = "right-down")]
    RightDown,
    #[nserde(rename = "right-up")]
    RightUp,
    #[nserde(rename = "left-down")]
    LeftDown,
    #[nserde(rename = "left-up")]
    LeftUp,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub enum Axis {
    #[default]
    #[nserde(rename = "x")]
    X,
    #[nserde(rename = "y")]
    Y,
    #[nserde(rename = "z")]
    Z,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub enum Parity {
    #[default]
    #[nserde(rename = "odd")]
    Odd,
    #[nserde(rename = "even")]
    Even,
}

impl Map {
    pub fn new(
        class: Option<String>,
        orientation: MapOrientation,
        width: u32,
        height: u32,
        tilewidth: u32,
        tileheight: u32,
    ) -> Self {
        Self {
            tiledversion: TILED_VERSION.to_string(),
            version: MAP_VERSION.to_string(),
            class,
            orientation,
            width,
            height,
            tilewidth,
            tileheight,
            compressionlevel: -1,
            nextlayerid: 1,
            nextobjectid: 1,
            ..Default::default()
        }
    }

    pub fn next_layer_id(&mut self) -> u32 {
        let id = self.nextlayerid;
        self.nextlayerid += 1;
        id
    }

    pub fn next_object_id(&mut self) -> u32 {
        let id = self.nextobjectid;
        self.nextobjectid += 1;
        id
    }
}

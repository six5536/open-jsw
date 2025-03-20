#![allow(clippy::question_mark)]

use nanoserde::DeJson;

use super::{layer::Layer, property::Property};

/// https://doc.mapeditor.org/en/stable/reference/tmx-map-format/#tmx-tileset
/// Represents a Tileset in the map.
#[derive(Clone, Debug, Default, DeJson)]
#[nserde(default)]
pub struct Tileset {
    /// Hex-formatted color (#RRGGBB or #AARRGGBB) (optional).
    pub backgroundcolor: Option<String>,

    /// The class of the tileset (since 1.9, optional).
    pub class: Option<String>,

    /// The number of tile columns in the tileset.
    pub columns: i32,

    /// The fill mode to use when rendering tiles from this tileset: "stretch" (default) or "preserve-aspect-fit" (since 1.9).
    pub fillmode: Option<FillMode>,

    /// GID corresponding to the first tile in the set.
    pub firstgid: i32,

    /// Grid information (optional).
    pub grid: Option<Grid>,

    /// Image used for tiles in this set.
    pub image: Option<String>,

    /// Height of the source image in pixels.
    pub imageheight: Option<i32>,

    /// Width of the source image in pixels.
    pub imagewidth: Option<i32>,

    /// Buffer between the image edge and the first tile (in pixels).
    pub margin: i32,

    /// Name given to this tileset.
    pub name: String,

    /// Alignment to use for tile objects: "unspecified" (default), "topleft", "top", "topright", "left",
    /// "center", "right", "bottomleft", "bottom", or "bottomright" (since 1.4).
    pub objectalignment: Option<ObjectAlignment>,

    /// Array of properties (optional).
    pub properties: Option<Vec<Property>>,

    /// The external file that contains this tileset’s data (optional).
    pub source: Option<String>,

    /// Spacing between adjacent tiles in the image (in pixels).
    pub spacing: i32,

    /// Array of terrains (optional).
    pub terrains: Option<Vec<Terrain>>,

    /// The number of tiles in this tileset.
    pub tilecount: i32,

    /// The Tiled version used to save the file.
    pub tiledversion: String,

    /// Maximum height of tiles in this set.
    pub tileheight: i32,

    /// Tile offset (optional).
    pub tileoffset: Option<TileOffset>,

    /// The size to use when rendering tiles from this tileset on a tile layer: "tile" (default) or "grid" (since 1.9).
    pub tilerendersize: Option<TileRenderSize>,

    /// Array of tiles (optional).
    #[nserde(default)]
    pub tiles: Option<Vec<Tile>>,

    /// Maximum width of tiles in this set.
    pub tilewidth: i32,

    /// Allowed transformations (optional).
    pub transformations: Option<Transformations>,

    /// Hex-formatted transparent color (#RRGGBB) (optional).
    pub transparentcolor: Option<String>,

    /// Type of tileset (always "tileset" for tileset files, since 1.0).
    #[nserde(rename = "type")]
    pub typ: TilesetType,

    /// The JSON format version (previously a number, saved as a string since 1.6).
    pub version: String,

    /// Array of Wang sets (since 1.1.5).
    pub wangsets: Option<Vec<WangSet>>,
}

/// Represents grid information in the tileset.
#[derive(Clone, Debug, Default, DeJson)]
pub struct Grid {
    /// Cell height of the tile grid.
    pub height: i32,

    /// Grid orientation: "orthogonal" (default) or "isometric".
    pub orientation: GridOrientation,

    /// Cell width of the tile grid.
    pub width: i32,
}

/// Represents tile offset information.
#[derive(Clone, Debug, Default, DeJson)]
pub struct TileOffset {
    /// Horizontal offset in pixels.
    pub x: i32,

    /// Vertical offset in pixels (positive is down).
    pub y: i32,
}

/// Represents allowed transformations in a tileset.
#[derive(Clone, Debug, Default, DeJson)]
pub struct Transformations {
    /// Whether tiles can be flipped horizontally.
    pub hflip: bool,

    /// Whether tiles can be flipped vertically.
    pub vflip: bool,

    /// Whether tiles can be rotated in 90-degree increments.
    pub rotate: bool,

    /// Whether untransformed tiles remain preferred; otherwise, transformed tiles are used to produce more variations.
    pub preferuntransformed: bool,
}

/// Represents a tile in the tileset.
#[derive(Clone, Debug, Default, DeJson)]
#[nserde(default)]
pub struct Tile {
    /// Array of frames for tile animation (optional).
    pub animation: Option<Vec<Frame>>,

    /// Local ID of the tile.
    pub id: i32,

    /// Image representing this tile (optional, used for image collection tilesets).
    pub image: Option<String>,

    /// Height of the tile image in pixels.
    pub imageheight: Option<i32>,

    /// Width of the tile image in pixels.
    pub imagewidth: Option<i32>,

    /// The X position of the sub-rectangle representing this tile (default: 0).
    pub x: Option<i32>,

    /// The Y position of the sub-rectangle representing this tile (default: 0).
    pub y: Option<i32>,

    /// The width of the sub-rectangle representing this tile (defaults to the image width).
    pub width: Option<i32>,

    /// The height of the sub-rectangle representing this tile (defaults to the image height).
    pub height: Option<i32>,

    /// Layer with type `objectgroup`, when collision shapes are specified (optional).
    pub objectgroup: Option<Layer>,

    /// Percentage chance this tile is chosen when competing with others in the editor (optional).
    pub probability: Option<f64>,

    /// Array of properties (optional).
    pub properties: Option<Vec<Property>>,

    /// Index of terrain for each corner of the tile (optional, replaced by Wang sets since 1.5).
    pub terrain: Option<Vec<i32>>,

    /// The class of the tile (was saved as `class` in 1.9, optional).
    #[nserde(rename = "type")]
    #[nserde(rename = "class")]
    pub typ: Option<String>,
}

/// Represents a Frame in an animated tile.
#[derive(Clone, Debug, Default, DeJson)]
pub struct Frame {
    /// Local tile ID representing a frame.
    pub tileid: i32,

    /// Duration in milliseconds for this frame.
    pub duration: i32,
}

/// Represents a Terrain in a Tileset.
#[derive(Clone, Debug, Default, DeJson)]
pub struct Terrain {
    /// Name of the terrain.
    pub name: String,

    /// Array of properties (optional).
    pub properties: Option<Vec<Property>>,

    /// Local ID of the tile representing this terrain.
    pub tile: i32,
}

/// Represents a Wang set in the tileset.
#[derive(Clone, Debug, Default, DeJson)]
pub struct WangSet {
    /// The class of the Wang set (since 1.9, optional).
    pub class: Option<String>,

    /// Array of Wang colors (since 1.5).
    pub colors: Vec<WangColor>,

    /// Name of the Wang set.
    pub name: String,

    /// Array of properties (optional).
    pub properties: Option<Vec<Property>>,

    /// Local ID of the tile representing the Wang set.
    pub tile: i32,

    /// Type of Wang set: "corner", "edge", or "mixed" (since 1.5).
    #[nserde(rename = "type")]
    pub typ: WangSetType,

    /// Array of Wang tiles.
    pub wangtiles: Vec<WangTile>,
}

/// Represents a Wang Color in a Wang Set.
#[derive(Clone, Debug, Default, DeJson)]
pub struct WangColor {
    /// The class of the Wang color (since 1.9, optional).
    pub class: Option<String>,

    /// Hex-formatted color (#RRGGBB or #AARRGGBB).
    pub color: String,

    /// Name of the Wang color.
    pub name: String,

    /// Probability used when randomizing.
    pub probability: f64,

    /// Array of properties (optional, since 1.5).
    pub properties: Option<Vec<Property>>,

    /// Local ID of the tile representing the Wang color.
    pub tile: i32,
}

/// Represents a Wang Tile in a Wang Set.
#[derive(Clone, Debug, Default, DeJson)]
pub struct WangTile {
    /// Local ID of the tile.
    pub tileid: i32,

    /// Array of Wang color indexes (8-bit unsigned integers).
    pub wangid: [u8; 8],
}

#[derive(Clone, Debug, Default, DeJson)]
pub enum FillMode {
    #[default]
    #[nserde(rename = "stretch")]
    Stretch,
    #[nserde(rename = "preserve-aspect-fit")]
    PreserveAspectFit,
}

#[derive(Clone, Debug, Default, DeJson)]
pub enum ObjectAlignment {
    #[default]
    #[nserde(rename = "unspecified")]
    Unspecified,
    #[nserde(rename = "topleft")]
    TopLeft,
    #[nserde(rename = "top")]
    Top,
    #[nserde(rename = "topright")]
    TopRight,
    #[nserde(rename = "left")]
    Left,
    #[nserde(rename = "center")]
    Center,
    #[nserde(rename = "right")]
    Right,
    #[nserde(rename = "bottomleft")]
    BottomLeft,
    #[nserde(rename = "bottom")]
    Bottom,
    #[nserde(rename = "bottomright")]
    BottomRight,
}

#[derive(Clone, Debug, Default, DeJson)]
pub enum TileRenderSize {
    #[default]
    #[nserde(rename = "tile")]
    Tile,
    #[nserde(rename = "grid")]
    Grid,
}

#[derive(Clone, Debug, Default, DeJson)]
pub enum TilesetType {
    #[default]
    #[nserde(rename = "tileset")]
    Tileset,
}

#[derive(Clone, Debug, Default, DeJson)]
pub enum GridOrientation {
    #[default]
    #[nserde(rename = "orthogonal")]
    Orthogonal,
    #[nserde(rename = "isometric")]
    Isometric,
}

#[derive(Clone, Debug, Default, DeJson)]
pub enum WangSetType {
    #[default]
    #[nserde(rename = "corner")]
    Corner,
    #[nserde(rename = "edge")]
    Edge,
    #[nserde(rename = "mixed")]
    Mixed,
}

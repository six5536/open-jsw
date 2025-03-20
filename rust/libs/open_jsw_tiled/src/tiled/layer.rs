#![allow(clippy::question_mark)]

use super::{chunk::Chunk, object::Object, property::Property};
use nanoserde::DeJson;

/// Represents a Layer in the map.
#[derive(Clone, Debug, Default, DeJson)]
#[nserde(default)]
pub struct Layer {
    /// Array of chunks (optional, tilelayer only).
    pub chunks: Option<Vec<Chunk>>,

    /// The class of the layer (since 1.9, optional).
    pub class: Option<String>,

    /// Compression type: "zlib", "gzip", "zstd" (since 1.3), or empty (default, tilelayer only).
    pub compression: Option<Compression>,

    /// Array of unsigned int (GIDs) or base64-encoded data (tilelayer only).
    // pub data: Option<Data>,
    pub data: Option<Vec<u32>>,

    /// Draw order: "topdown" (default) or "index" (objectgroup only).
    pub draworder: Option<DrawOrder>,

    /// Encoding type: "csv" (default) or "base64" (tilelayer only).
    pub encoding: Option<LayerEncoding>,

    /// Row count, same as map height for fixed-size maps (tilelayer only).
    pub height: Option<i32>,

    /// Unique incremental ID across all layers.
    pub id: i32,

    /// Image used by this layer (imagelayer only).
    pub image: Option<String>,

    /// Height of the image used by this layer (imagelayer only, since 1.11.1).
    pub imageheight: Option<i32>,

    /// Width of the image used by this layer (imagelayer only, since 1.11.1).
    pub imagewidth: Option<i32>,

    /// Array of sub-layers (group only).
    pub layers: Option<Vec<Layer>>,

    /// Whether the layer is locked in the editor (default: false, since 1.8.2).
    #[nserde(default)]
    pub locked: bool,

    /// Name assigned to this layer.
    pub name: String,

    /// Array of objects (objectgroup only).
    #[nserde(default)]
    pub objects: Option<Vec<Object>>,

    /// Horizontal layer offset in pixels (default: 0).
    pub offsetx: f64,

    /// Vertical layer offset in pixels (default: 0).
    pub offsety: f64,

    /// Opacity value between 0 and 1.
    pub opacity: f64,

    /// Horizontal parallax factor for this layer (default: 1, since 1.5).
    pub parallaxx: f64,

    /// Vertical parallax factor for this layer (default: 1, since 1.5).
    pub parallaxy: f64,

    /// Array of properties (optional).
    pub properties: Option<Vec<Property>>,

    /// Whether the image drawn by this layer is repeated along the X axis (imagelayer only, since 1.8).
    pub repeatx: Option<bool>,

    /// Whether the image drawn by this layer is repeated along the Y axis (imagelayer only, since 1.8).
    pub repeaty: Option<bool>,

    /// X coordinate where layer content starts (for infinite maps).
    pub startx: Option<i32>,

    /// Y coordinate where layer content starts (for infinite maps).
    pub starty: Option<i32>,

    /// Hex-formatted tint color (#RRGGBB or #AARRGGBB) that is multiplied with any graphics drawn by this layer or any child layers (optional).
    pub tintcolor: Option<String>,

    /// Hex-formatted transparent color (#RRGGBB) (optional, imagelayer only).
    pub transparentcolor: Option<String>,

    /// Type of layer: "tilelayer", "objectgroup", "imagelayer", or "group".
    #[nserde(rename = "type")]
    pub typ: LayerType,

    /// Whether the layer is visible in the editor.
    pub visible: bool,

    /// Column count, same as map width for fixed-size maps (tilelayer only).
    pub width: Option<i32>,

    /// Horizontal layer offset in tiles (always 0).
    pub x: i32,

    /// Vertical layer offset in tiles (always 0).
    pub y: i32,
}

// /// Represents a Layer in the map.
// #[derive(Clone, Debug, DeJson)]
// #[nserde(default)]
// #[nserde(tag = "type")]
// pub enum Layer {
//     #[nserde(rename = "tilelayer")]
//     TileLayer {
//         /// Array of chunks (optional, tilelayer only).
//         chunks: Option<Vec<Chunk>>,

//         /// The class of the layer (since 1.9, optional).
//         class: Option<String>,

//         /// Compression type: "zlib", "gzip", "zstd" (since 1.3), or empty (default, tilelayer only).
//         compression: Option<Compression>,

//         /// Array of unsigned int (GIDs) or base64-encoded data (tilelayer only).
//         // data: Option<Data>,
//         data: Option<Vec<u32>>,

//         /// Draw order: "topdown" (default) or "index" (objectgroup only).
//         draworder: Option<DrawOrder>,

//         /// Encoding type: "csv" (default) or "base64" (tilelayer only).
//         encoding: Option<LayerEncoding>,

//         /// Row count, same as map height for fixed-size maps (tilelayer only).
//         height: Option<i32>,

//         /// Unique incremental ID across all layers.
//         id: i32,

//         /// Image used by this layer (imagelayer only).
//         image: Option<String>,

//         /// Height of the image used by this layer (imagelayer only, since 1.11.1).
//         imageheight: Option<i32>,

//         /// Width of the image used by this layer (imagelayer only, since 1.11.1).
//         imagewidth: Option<i32>,

//         /// Array of sub-layers (group only).
//         layers: Option<Vec<Layer>>,

//         /// Whether the layer is locked in the editor (default: false, since 1.8.2).
//         #[nserde(default)]
//         locked: bool,

//         /// Name assigned to this layer.
//         name: String,

//         /// Array of objects (objectgroup only).
//         #[nserde(default)]
//         objects: Option<Vec<Object>>,

//         /// Horizontal layer offset in pixels (default: 0).
//         offsetx: f64,

//         /// Vertical layer offset in pixels (default: 0).
//         offsety: f64,

//         /// Opacity value between 0 and 1.
//         opacity: f64,

//         /// Horizontal parallax factor for this layer (default: 1, since 1.5).
//         parallaxx: f64,

//         /// Vertical parallax factor for this layer (default: 1, since 1.5).
//         parallaxy: f64,

//         /// Array of properties (optional).
//         properties: Option<Vec<Property>>,

//         /// Whether the image drawn by this layer is repeated along the X axis (imagelayer only, since 1.8).
//         repeatx: Option<bool>,

//         /// Whether the image drawn by this layer is repeated along the Y axis (imagelayer only, since 1.8).
//         repeaty: Option<bool>,

//         /// X coordinate where layer content starts (for infinite maps).
//         startx: Option<i32>,

//         /// Y coordinate where layer content starts (for infinite maps).
//         starty: Option<i32>,

//         /// Hex-formatted tint color (#RRGGBB or #AARRGGBB) that is multiplied with any graphics drawn by this layer or any child layers (optional).
//         tintcolor: Option<String>,

//         /// Hex-formatted transparent color (#RRGGBB) (optional, imagelayer only).
//         transparentcolor: Option<String>,

//         /// Type of layer: "tilelayer", "objectgroup", "imagelayer", or "group".
//         #[nserde(rename = "type")]
//         typ: LayerType,

//         /// Whether the layer is visible in the editor.
//         visible: bool,

//         /// Column count, same as map width for fixed-size maps (tilelayer only).
//         width: Option<i32>,

//         /// Horizontal layer offset in tiles (always 0).
//         x: i32,

//         /// Vertical layer offset in tiles (always 0).
//         y: i32,
//     },
//     #[nserde(rename = "objectgroup")]
//     ObjectGroup {
//         /// Array of chunks (optional, tilelayer only).
//         chunks: Option<Vec<Chunk>>,

//         /// The class of the layer (since 1.9, optional).
//         class: Option<String>,

//         /// Compression type: "zlib", "gzip", "zstd" (since 1.3), or empty (default, tilelayer only).
//         compression: Option<Compression>,

//         /// Array of unsigned int (GIDs) or base64-encoded data (tilelayer only).
//         // data: Option<Data>,
//         data: Option<Vec<u32>>,

//         /// Draw order: "topdown" (default) or "index" (objectgroup only).
//         draworder: Option<DrawOrder>,

//         /// Encoding type: "csv" (default) or "base64" (tilelayer only).
//         encoding: Option<LayerEncoding>,

//         /// Row count, same as map height for fixed-size maps (tilelayer only).
//         height: Option<i32>,

//         /// Unique incremental ID across all layers.
//         id: i32,

//         /// Image used by this layer (imagelayer only).
//         image: Option<String>,

//         /// Height of the image used by this layer (imagelayer only, since 1.11.1).
//         imageheight: Option<i32>,

//         /// Width of the image used by this layer (imagelayer only, since 1.11.1).
//         imagewidth: Option<i32>,

//         /// Array of sub-layers (group only).
//         layers: Option<Vec<Layer>>,

//         /// Whether the layer is locked in the editor (default: false, since 1.8.2).
//         #[nserde(default)]
//         locked: bool,

//         /// Name assigned to this layer.
//         name: String,

//         /// Array of objects (objectgroup only).
//         #[nserde(default)]
//         objects: Option<Vec<Object>>,

//         /// Horizontal layer offset in pixels (default: 0).
//         offsetx: f64,

//         /// Vertical layer offset in pixels (default: 0).
//         offsety: f64,

//         /// Opacity value between 0 and 1.
//         opacity: f64,

//         /// Horizontal parallax factor for this layer (default: 1, since 1.5).
//         parallaxx: f64,

//         /// Vertical parallax factor for this layer (default: 1, since 1.5).
//         parallaxy: f64,

//         /// Array of properties (optional).
//         properties: Option<Vec<Property>>,

//         /// Whether the image drawn by this layer is repeated along the X axis (imagelayer only, since 1.8).
//         repeatx: Option<bool>,

//         /// Whether the image drawn by this layer is repeated along the Y axis (imagelayer only, since 1.8).
//         repeaty: Option<bool>,

//         /// X coordinate where layer content starts (for infinite maps).
//         startx: Option<i32>,

//         /// Y coordinate where layer content starts (for infinite maps).
//         starty: Option<i32>,

//         /// Hex-formatted tint color (#RRGGBB or #AARRGGBB) that is multiplied with any graphics drawn by this layer or any child layers (optional).
//         tintcolor: Option<String>,

//         /// Hex-formatted transparent color (#RRGGBB) (optional, imagelayer only).
//         transparentcolor: Option<String>,

//         /// Type of layer: "tilelayer", "objectgroup", "imagelayer", or "group".
//         #[nserde(rename = "type")]
//         typ: LayerType,

//         /// Whether the layer is visible in the editor.
//         visible: bool,

//         /// Column count, same as map width for fixed-size maps (tilelayer only).
//         width: Option<i32>,

//         /// Horizontal layer offset in tiles (always 0).
//         x: i32,

//         /// Vertical layer offset in tiles (always 0).
//         y: i32,
//     },
//     #[nserde(rename = "imagelayer")]
//     ImageLayer {
//         /// Array of chunks (optional, tilelayer only).
//         chunks: Option<Vec<Chunk>>,

//         /// The class of the layer (since 1.9, optional).
//         class: Option<String>,

//         /// Compression type: "zlib", "gzip", "zstd" (since 1.3), or empty (default, tilelayer only).
//         compression: Option<Compression>,

//         /// Array of unsigned int (GIDs) or base64-encoded data (tilelayer only).
//         // data: Option<Data>,
//         data: Option<Vec<u32>>,

//         /// Draw order: "topdown" (default) or "index" (objectgroup only).
//         draworder: Option<DrawOrder>,

//         /// Encoding type: "csv" (default) or "base64" (tilelayer only).
//         encoding: Option<LayerEncoding>,

//         /// Row count, same as map height for fixed-size maps (tilelayer only).
//         height: Option<i32>,

//         /// Unique incremental ID across all layers.
//         id: i32,

//         /// Image used by this layer (imagelayer only).
//         image: Option<String>,

//         /// Height of the image used by this layer (imagelayer only, since 1.11.1).
//         imageheight: Option<i32>,

//         /// Width of the image used by this layer (imagelayer only, since 1.11.1).
//         imagewidth: Option<i32>,

//         /// Array of sub-layers (group only).
//         layers: Option<Vec<Layer>>,

//         /// Whether the layer is locked in the editor (default: false, since 1.8.2).
//         #[nserde(default)]
//         locked: bool,

//         /// Name assigned to this layer.
//         name: String,

//         /// Array of objects (objectgroup only).
//         #[nserde(default)]
//         objects: Option<Vec<Object>>,

//         /// Horizontal layer offset in pixels (default: 0).
//         offsetx: f64,

//         /// Vertical layer offset in pixels (default: 0).
//         offsety: f64,

//         /// Opacity value between 0 and 1.
//         opacity: f64,

//         /// Horizontal parallax factor for this layer (default: 1, since 1.5).
//         parallaxx: f64,

//         /// Vertical parallax factor for this layer (default: 1, since 1.5).
//         parallaxy: f64,

//         /// Array of properties (optional).
//         properties: Option<Vec<Property>>,

//         /// Whether the image drawn by this layer is repeated along the X axis (imagelayer only, since 1.8).
//         repeatx: Option<bool>,

//         /// Whether the image drawn by this layer is repeated along the Y axis (imagelayer only, since 1.8).
//         repeaty: Option<bool>,

//         /// X coordinate where layer content starts (for infinite maps).
//         startx: Option<i32>,

//         /// Y coordinate where layer content starts (for infinite maps).
//         starty: Option<i32>,

//         /// Hex-formatted tint color (#RRGGBB or #AARRGGBB) that is multiplied with any graphics drawn by this layer or any child layers (optional).
//         tintcolor: Option<String>,

//         /// Hex-formatted transparent color (#RRGGBB) (optional, imagelayer only).
//         transparentcolor: Option<String>,

//         /// Type of layer: "tilelayer", "objectgroup", "imagelayer", or "group".
//         #[nserde(rename = "type")]
//         typ: LayerType,

//         /// Whether the layer is visible in the editor.
//         visible: bool,

//         /// Column count, same as map width for fixed-size maps (tilelayer only).
//         width: Option<i32>,

//         /// Horizontal layer offset in tiles (always 0).
//         x: i32,

//         /// Vertical layer offset in tiles (always 0).
//         y: i32,
//     },
//     #[nserde(rename = "group")]
//     Group {
//         /// Array of chunks (optional, tilelayer only).
//         chunks: Option<Vec<Chunk>>,

//         /// The class of the layer (since 1.9, optional).
//         class: Option<String>,

//         /// Compression type: "zlib", "gzip", "zstd" (since 1.3), or empty (default, tilelayer only).
//         compression: Option<Compression>,

//         /// Array of unsigned int (GIDs) or base64-encoded data (tilelayer only).
//         // data: Option<Data>,
//         data: Option<Vec<u32>>,

//         /// Draw order: "topdown" (default) or "index" (objectgroup only).
//         draworder: Option<DrawOrder>,

//         /// Encoding type: "csv" (default) or "base64" (tilelayer only).
//         encoding: Option<LayerEncoding>,

//         /// Row count, same as map height for fixed-size maps (tilelayer only).
//         height: Option<i32>,

//         /// Unique incremental ID across all layers.
//         id: i32,

//         /// Image used by this layer (imagelayer only).
//         image: Option<String>,

//         /// Height of the image used by this layer (imagelayer only, since 1.11.1).
//         imageheight: Option<i32>,

//         /// Width of the image used by this layer (imagelayer only, since 1.11.1).
//         imagewidth: Option<i32>,

//         /// Array of sub-layers (group only).
//         layers: Option<Vec<Layer>>,

//         /// Whether the layer is locked in the editor (default: false, since 1.8.2).
//         #[nserde(default)]
//         locked: bool,

//         /// Name assigned to this layer.
//         name: String,

//         /// Array of objects (objectgroup only).
//         #[nserde(default)]
//         objects: Option<Vec<Object>>,

//         /// Horizontal layer offset in pixels (default: 0).
//         offsetx: f64,

//         /// Vertical layer offset in pixels (default: 0).
//         offsety: f64,

//         /// Opacity value between 0 and 1.
//         opacity: f64,

//         /// Horizontal parallax factor for this layer (default: 1, since 1.5).
//         parallaxx: f64,

//         /// Vertical parallax factor for this layer (default: 1, since 1.5).
//         parallaxy: f64,

//         /// Array of properties (optional).
//         properties: Option<Vec<Property>>,

//         /// Whether the image drawn by this layer is repeated along the X axis (imagelayer only, since 1.8).
//         repeatx: Option<bool>,

//         /// Whether the image drawn by this layer is repeated along the Y axis (imagelayer only, since 1.8).
//         repeaty: Option<bool>,

//         /// X coordinate where layer content starts (for infinite maps).
//         startx: Option<i32>,

//         /// Y coordinate where layer content starts (for infinite maps).
//         starty: Option<i32>,

//         /// Hex-formatted tint color (#RRGGBB or #AARRGGBB) that is multiplied with any graphics drawn by this layer or any child layers (optional).
//         tintcolor: Option<String>,

//         /// Hex-formatted transparent color (#RRGGBB) (optional, imagelayer only).
//         transparentcolor: Option<String>,

//         /// Type of layer: "tilelayer", "objectgroup", "imagelayer", or "group".
//         #[nserde(rename = "type")]
//         typ: LayerType,

//         /// Whether the layer is visible in the editor.
//         visible: bool,

//         /// Column count, same as map width for fixed-size maps (tilelayer only).
//         width: Option<i32>,

//         /// Horizontal layer offset in tiles (always 0).
//         x: i32,

//         /// Vertical layer offset in tiles (always 0).
//         y: i32,
//     },
// }

#[derive(Clone, Debug, Default, DeJson)]
pub enum LayerType {
    #[default]
    #[nserde(rename = "tilelayer")]
    TileLayer,
    #[nserde(rename = "objectgroup")]
    ObjectGroup,
    #[nserde(rename = "imagelayer")]
    ImageLayer,
    #[nserde(rename = "group")]
    Group,
}

#[derive(Clone, Debug, DeJson)]
pub enum Compression {
    #[nserde(rename = "zlib")]
    Zlib,
    #[nserde(rename = "gzip")]
    Gzip,
    #[nserde(rename = "zstd")]
    Zstd,
}

#[derive(Clone, Debug, Default, DeJson)]
pub enum DrawOrder {
    #[default]
    #[nserde(rename = "topdown")]
    TopDown,
    #[nserde(rename = "index")]
    Index,
}

#[derive(Clone, Debug, Default, DeJson)]
pub enum LayerEncoding {
    #[default]
    #[nserde(rename = "csv")]
    Csv,
    #[nserde(rename = "base64")]
    Base64,
}

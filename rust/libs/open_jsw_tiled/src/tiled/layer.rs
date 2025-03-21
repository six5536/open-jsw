#![allow(clippy::question_mark)]

use super::{chunk::Chunk, layer_data::TileMatrix, map::Map, object::Object, property::Property};
use nanoserde::{DeJson, SerJson};

/// Represents a Layer in the map.
#[derive(Clone, Debug, Default, DeJson, SerJson)]
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
    pub height: Option<u32>,

    /// Unique incremental ID across all layers.
    pub id: u32,

    /// Image used by this layer (imagelayer only).
    pub image: Option<String>,

    /// Height of the image used by this layer (imagelayer only, since 1.11.1).
    pub imageheight: Option<u32>,

    /// Width of the image used by this layer (imagelayer only, since 1.11.1).
    pub imagewidth: Option<u32>,

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
    pub width: Option<u32>,

    /// Horizontal layer offset in tiles (always 0).
    pub x: i32,

    /// Vertical layer offset in tiles (always 0).
    pub y: i32,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
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

#[derive(Clone, Debug, DeJson, SerJson)]
pub enum Compression {
    #[nserde(rename = "zlib")]
    Zlib,
    #[nserde(rename = "gzip")]
    Gzip,
    #[nserde(rename = "zstd")]
    Zstd,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub enum DrawOrder {
    #[default]
    #[nserde(rename = "topdown")]
    TopDown,
    #[nserde(rename = "index")]
    Index,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub enum LayerEncoding {
    #[default]
    #[nserde(rename = "csv")]
    Csv,
    #[nserde(rename = "base64")]
    Base64,
}

impl Layer {
    pub fn new(map: &mut Map, typ: LayerType, name: String) -> Self {
        let id = map.next_layer_id();

        let mut data = None;
        let mut width = None;
        let mut height = None;

        match typ {
            LayerType::TileLayer => {
                data = Some(vec![0; (map.width * map.height) as usize]);
                width = Some(map.width);
                height = Some(map.height);
            }
            LayerType::ObjectGroup => (),
            LayerType::ImageLayer => (),
            LayerType::Group => (),
        }

        Self {
            id,
            typ,
            name,
            encoding: Some(LayerEncoding::Csv),
            compression: None,
            opacity: 1.0,
            parallaxx: 1.0,
            parallaxy: 1.0,
            height,
            width,
            data,
            ..Default::default()
        }
    }

    pub fn get_tile_matrix(&mut self) -> TileMatrix<u32> {
        match self.typ {
            LayerType::TileLayer => {
                let data = self.data.as_mut().unwrap();
                TileMatrix::wrap_vec(data, self.width.unwrap() as usize)
            }
            _ => panic!("Layer is not a tile layer"),
        }
    }
}

#![allow(clippy::question_mark)]

use nanoserde::DeJson;

use super::{object::Object, tileset::Tileset};

/// Represents a Template in Tiled.
#[derive(Clone, Debug, Default, DeJson)]
#[nserde(default)]
pub struct ObjectTemplate {
    /// Type of the template.
    #[nserde(rename = "type")]
    pub typ: String,

    /// External tileset used by the template (optional).
    pub tileset: Option<Tileset>,

    /// The object instantiated by this template.
    pub object: Object,
}

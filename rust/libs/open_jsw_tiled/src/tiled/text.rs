use nanoserde::DeJson;

/// Represents a Text object in Tiled.
#[derive(Clone, Debug, Default, DeJson)]
#[nserde(default)]
pub struct Text {
    /// Whether to use a bold font (default: false).
    pub bold: bool,

    /// Hex-formatted color (#RRGGBB or #AARRGGBB) (default: #000000).
    pub color: String,

    /// Font family (default: sans-serif).
    pub fontfamily: String,

    /// Horizontal alignment: "center", "right", "justify", or "left" (default).
    pub halign: HorizontalAlignment,

    /// Whether to use an italic font (default: false).
    pub italic: bool,

    /// Whether to use kerning when placing characters (default: true).
    pub kerning: bool,

    /// Pixel size of the font (default: 16).
    pub pixelsize: i32,

    /// Whether to strike out the text (default: false).
    pub strikeout: bool,

    /// The actual text content.
    pub text: String,

    /// Whether to underline the text (default: false).
    pub underline: bool,

    /// Vertical alignment: "center", "bottom", or "top" (default).
    pub valign: VerticalAlignment,

    /// Whether the text is wrapped within the object bounds (default: false).
    pub wrap: bool,
}

#[derive(Clone, Debug, Default, DeJson)]
pub enum HorizontalAlignment {
    #[nserde(rename = "center")]
    Center,
    #[nserde(rename = "right")]
    Right,
    #[nserde(rename = "justify")]
    Justify,
    #[default]
    #[nserde(rename = "left")]
    Left,
}

#[derive(Clone, Debug, Default, DeJson)]
pub enum VerticalAlignment {
    #[nserde(rename = "center")]
    Center,
    #[nserde(rename = "bottom")]
    Bottom,
    #[default]
    #[nserde(rename = "top")]
    Top,
}

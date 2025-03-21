use nanoserde::{DeJson, SerJson};

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

use macroquad::color::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum SpeccyColour {
    #[default]
    Black = 0,
    Blue,
    Red,
    Magenta,
    Green,
    Cyan,
    Yellow,
    White,
}

impl SpeccyColour {
    pub fn from_raw(n: u8) -> SpeccyColour {
        match n {
            0 => SpeccyColour::Black,
            1 => SpeccyColour::Blue,
            2 => SpeccyColour::Red,
            3 => SpeccyColour::Magenta,
            4 => SpeccyColour::Green,
            5 => SpeccyColour::Cyan,
            6 => SpeccyColour::Yellow,
            7 => SpeccyColour::White,
            _ => SpeccyColour::default(),
        }
    }

    pub fn to_rgba(&self, bright: bool) -> Color {
        match self {
            SpeccyColour::Black => Color::from_hex(0x000000),
            SpeccyColour::Blue => {
                if bright {
                    Color::from_hex(0x0000ff)
                } else {
                    Color::from_hex(0x0000cd)
                }
            }
            SpeccyColour::Red => {
                if bright {
                    Color::from_hex(0xff0000)
                } else {
                    Color::from_hex(0xcd0000)
                }
            }
            SpeccyColour::Magenta => {
                if bright {
                    Color::from_hex(0xff00ff)
                } else {
                    Color::from_hex(0xcd00cd)
                }
            }
            SpeccyColour::Green => {
                if bright {
                    Color::from_hex(0x00ff00)
                } else {
                    Color::from_hex(0x00cd00)
                }
            }
            SpeccyColour::Cyan => {
                if bright {
                    Color::from_hex(0x00ffff)
                } else {
                    Color::from_hex(0x00cdcd)
                }
            }
            SpeccyColour::Yellow => {
                if bright {
                    Color::from_hex(0xffff00)
                } else {
                    Color::from_hex(0xcdcd00)
                }
            }
            SpeccyColour::White => {
                if bright {
                    Color::from_hex(0xffffff)
                } else {
                    Color::from_hex(0xcdcdcd)
                }
            }
        }
    }
}

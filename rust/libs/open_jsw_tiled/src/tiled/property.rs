use nanoserde::{DeJson, SerJson};

pub mod property_type {
    pub const STRING: &str = "string";
    pub const INT: &str = "int";
    pub const FLOAT: &str = "float";
    pub const BOOL: &str = "bool";
    pub const COLOR: &str = "color";
    pub const FILE: &str = "file";
    pub const OBJECT: &str = "object";
    pub const ARRAY: &str = "array";
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct Property {
    pub name: String,
    pub value: PropertyVal,
    #[nserde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Clone)]
pub enum PropertyVal {
    String(String),
    UInt(u64),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl Default for PropertyVal {
    fn default() -> Self {
        PropertyVal::Boolean(false)
    }
}

impl std::fmt::Display for PropertyVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyVal::String(x) => write!(f, "{}", x),
            PropertyVal::UInt(x) => write!(f, "{}", x),
            PropertyVal::Integer(x) => write!(f, "{}", x),
            PropertyVal::Float(x) => write!(f, "{}", x),
            PropertyVal::Boolean(x) => write!(f, "{}", x),
        }
    }
}

impl DeJson for PropertyVal {
    fn de_json(
        s: &mut nanoserde::DeJsonState,
        i: &mut std::str::Chars,
    ) -> Result<Self, nanoserde::DeJsonErr> {
        use nanoserde::DeJsonTok;

        let v = match s.tok {
            DeJsonTok::Bool(b) => PropertyVal::Boolean(b),
            DeJsonTok::U64(x) => PropertyVal::UInt(x),
            DeJsonTok::I64(x) => PropertyVal::Integer(x),
            DeJsonTok::F64(x) => PropertyVal::Float(x),
            #[allow(clippy::mem_replace_with_default)]
            DeJsonTok::Str => PropertyVal::String(core::mem::replace(&mut s.strbuf, String::new())),
            _ => {
                return Err(s.err_token(
                    "Incorrect property value. Must be either string, number or boolean",
                ));
            }
        };

        s.next_tok(i)?;

        Ok(v)
    }
}

// &self, d: usize, s: &mut SerJsonState

impl SerJson for PropertyVal {
    fn ser_json(&self, _d: usize, _s: &mut nanoserde::SerJsonState) {
        // TODO: Implement this
        // use nanoserde::DeJsonTok;

        // let v = match s.tok {
        //     DeJsonTok::Bool(b) => PropertyVal::Boolean(b),
        //     DeJsonTok::U64(x) => PropertyVal::UInt(x),
        //     DeJsonTok::I64(x) => PropertyVal::Integer(x),
        //     DeJsonTok::F64(x) => PropertyVal::Float(x),
        //     #[allow(clippy::mem_replace_with_default)]
        //     DeJsonTok::Str => PropertyVal::String(core::mem::replace(&mut s.strbuf, String::new())),
        //     _ => {
        //         return Err(s.err_token(
        //             "Incorrect property value. Must be either string, number or boolean",
        //         ));
        //     }
        // };

        // s.next_tok(i)?;

        // Ok(v)
    }
}

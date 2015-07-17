#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;

pub use std::collections::BTreeMap as Map;
use serde::{Serialize, Serializer};
pub type Int = i64;
pub type Double = f64;

#[derive(Serialize, Deserialize)]
pub struct Pandoc(Meta, Vec<Block>);

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Meta {
    unMeta: Map<String, MetaValue>,
}

#[derive(Deserialize)]
pub enum MetaValue {
    MetaMap(Map<String, Box<MetaValue>>),
    MetaList(Vec<MetaValue>),
    MetaBool(bool),
    MetaString(String),
    MetaInlines(Vec<Inline>),
    MetaBlocks(Vec<Block>),
}

#[derive(Serialize)]
struct Helper<T: Serialize> {
    t: &'static str,
    c: T,
}

macro_rules! seq {
    ($ser:expr, $var:expr, $vec:expr) => (
        Helper {
            t: $var,
            c: $vec,
        }.serialize($ser)
    )
}

impl Serialize for MetaValue {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error> where S: Serializer {
        use self::MetaValue::*;
        match *self {
            MetaMap(ref val) => seq!(ser, "MetaMap", vec![val]),
            MetaList(ref val) => seq!(ser, "MetaList", val),
            MetaBool(ref val) => seq!(ser, "MetaBool", vec![val]),
            MetaString(ref val) => seq!(ser, "MetaString", vec![val]),
            MetaInlines(ref val) => seq!(ser, "MetaInlines", val),
            MetaBlocks(ref val) => seq!(ser, "MetaBlocks", val),
        }
    }
}

#[derive(Deserialize)]
pub enum Block {
    Plain(Vec<Inline>),
    Para(Vec<Inline>),
    CodeBlock(Attr, String),
    RawBlock(Format, String),
    BlockQuote(Vec<Block>),
    OrderedList(ListAttributes, Vec<Vec<Block>>),
    BulletList(Vec<Vec<Block>>),
    DefinitionList(Vec<(Vec<Inline>, Vec<Vec<Block>>)>),
    Header(Int, Attr, Vec<Inline>),
    HorizontalRule,
    Table(Vec<Inline>, Vec<Alignment>, Vec<Double>, Vec<TableCell>, Vec<Vec<TableCell>>),
    Div(Attr, Vec<Block>),
    Null,
}

impl Serialize for Block {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error> where S: Serializer {
        use self::Block::*;
        match *self {
            Plain(ref val) => seq!(ser, "Plain", val),
            Para(ref val) => seq!(ser, "Para", val),
            CodeBlock(ref val, ref val2) => seq!(ser, "CodeBlock", (val, val2)),
            RawBlock(ref val, ref val2) => seq!(ser, "RawBlock", (val, val2)),
            BlockQuote(ref val) => seq!(ser, "BlockQuote", val),
            OrderedList(ref val, ref val2) => seq!(ser, "OrderedList", (val, val2)),
            BulletList(ref val) => seq!(ser, "BulletList", val),
            DefinitionList(ref val) => seq!(ser, "DefinitionList", val),
            Header(ref val, ref val2, ref val3) => seq!(ser, "Header", (val, val2, val3)),
            HorizontalRule => seq!(ser, "HorizontalRule", Unit),
            Table(ref val, ref v2, ref v3, ref v4, ref v5) => seq!(ser, "Table", (val, v2, v3, v4, v5)),
            Div(ref val, ref val2) => seq!(ser, "Div", (val, val2)),
            Null => seq!(ser, "Null", Unit),
        }
    }
}

#[derive(Deserialize)]
pub enum Inline {
    Str(String),
    Emph(Vec<Inline>),
    Strong(Vec<Inline>),
    Strikeout(Vec<Inline>),
    Superscript(Vec<Inline>),
    Subscript(Vec<Inline>),
    SmallCaps(Vec<Inline>),
    Quoted(QuoteType,Vec<Inline>),
    Cite(Vec<Citation>, Vec<Inline>),
    Code(Attr, String),
    Space,
    LineBreak,
    Math(MathType, String),
    RawInline(Format, String),
    Link(Vec<Inline>, Target),
    Image(Vec<Inline>, Target),
    Note(Vec<Block>),
    Span(Attr, Vec<Inline>),
}

impl Serialize for Inline {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error> where S: Serializer {
        use self::Inline::*;
        match *self {
            Str(ref val) => seq!(ser, "Str", val),
            Emph(ref val) => seq!(ser, "Emph", val),
            Strong(ref val) => seq!(ser, "Strong", val),
            Strikeout(ref val) => seq!(ser, "Strikeout", val),
            Superscript(ref val) => seq!(ser, "Superscript", val),
            Subscript(ref val) => seq!(ser, "Subscript", val),
            SmallCaps(ref val) => seq!(ser, "SmallCaps", val),
            Quoted(ref val, ref val2) => seq!(ser, "Quoted", (val, val2)),
            Cite(ref val, ref val2) => seq!(ser, "Cite", (val, val2)),
            Code(ref val, ref val2) => seq!(ser, "Code", (val, val2)),
            Space => seq!(ser, "Space", Unit),
            LineBreak => seq!(ser, "LineBreak", Unit),
            Math(ref val, ref val2) => seq!(ser, "Math", (val, val2)),
            RawInline(ref val, ref val2) => seq!(ser, "RawInline", (val, val2)),
            Link(ref val, ref val2) => seq!(ser, "Link", (val, val2)),
            Image(ref val, ref val2) => seq!(ser, "Image", (val, val2)),
            Note(ref val) => seq!(ser, "Note", val),
            Span(ref val, ref val2) => seq!(ser, "Span", (val, val2)),
        }
    }
}

#[derive(Deserialize)]
pub enum Alignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
    AlignDefault,
}

impl Serialize for Alignment {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error> where S: Serializer {
        use self::Alignment::*;
        match *self {
            AlignLeft => seq!(ser, "AlignLeft", Unit),
            AlignRight => seq!(ser, "AlignRight", Unit),
            AlignCenter => seq!(ser, "AlignCenter", Unit),
            AlignDefault => seq!(ser, "AlignDefault", Unit),
        }
    }
}

pub type ListAttributes = (Int, ListNumberStyle, ListNumberDelim);

#[derive(Deserialize)]
pub enum ListNumberStyle {
    DefaultStyle,
    Example,
    Decimal,
    LowerRoman,
    UpperRoman,
    LowerAlpha,
    UpperAlpha,
}

impl Serialize for ListNumberStyle {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error> where S: Serializer {
        use self::ListNumberStyle::*;
        match *self {
            DefaultStyle => seq!(ser, "DefaultStyle", Unit),
            Example => seq!(ser, "Example", Unit),
            Decimal => seq!(ser, "Decimal", Unit),
            LowerRoman => seq!(ser, "LowerRoman", Unit),
            UpperRoman => seq!(ser, "UpperRoman", Unit),
            LowerAlpha => seq!(ser, "LowerAlpha", Unit),
            UpperAlpha => seq!(ser, "UpperAlpha", Unit),
        }
    }
}

#[derive(Deserialize)]
pub enum ListNumberDelim {
    DefaultDelim,
    Period,
    OneParen,
    TwoParens,
}

impl Serialize for ListNumberDelim {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error> where S: Serializer {
        use self::ListNumberDelim::*;
        match *self {
            DefaultDelim => seq!(ser, "DefaultDelim", Unit),
            Period => seq!(ser, "Period", Unit),
            OneParen => seq!(ser, "OneParen", Unit),
            TwoParens => seq!(ser, "TwoParens", Unit),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Format(String);

pub type Attr = (String, Vec<String>, Vec<(String, String)>);

pub type TableCell = Vec<Block>;

#[derive(Deserialize)]
pub enum QuoteType {
    SingleQuote,
    DoubleQuote,
}

impl Serialize for QuoteType {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error> where S: Serializer {
        use self::QuoteType::*;
        match *self {
            SingleQuote => seq!(ser, "SingleQuote", Unit),
            DoubleQuote => seq!(ser, "DoubleQuote", Unit),
        }
    }
}

struct Unit;

impl serde::ser::SeqVisitor for Unit {
    fn visit<S>(&mut self, _: &mut S) -> Result<Option<()>, S::Error>
        where S: Serializer {
        Ok(None)
    }
    fn len(&self) -> Option<usize> {
        Some(0)
    }
}

impl Serialize for Unit {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error> where S: Serializer {
        ser.visit_seq(Unit)
    }
}

pub type Target = (String, String);

#[derive(Deserialize)]
pub enum MathType {
    DisplayMath,
    InlineMath,
}

impl Serialize for MathType {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error> where S: Serializer {
        use self::MathType::*;
        match *self {
            DisplayMath => seq!(ser, "DisplayMath", Unit),
            InlineMath => seq!(ser, "InlineMath", Unit),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Citation {
    citationId: String,
    citationPrefix: Vec<Inline>,
    citationSuffix: Vec<Inline>,
    citationMode: CitationMode,
    citationNoteNum: Int,
    citationHash: Int,
}

#[derive(Deserialize)]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    NormalCitation,
}

impl Serialize for CitationMode {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error> where S: Serializer {
        use self::CitationMode::*;
        match *self {
            AuthorInText => seq!(ser, "AuthorInText", Unit),
            SuppressAuthor => seq!(ser, "SuppressAuthor", Unit),
            NormalCitation => seq!(ser, "NormalCitation", Unit),
        }
    }
}
use serde::json::{Value, from_str, to_string, from_value};

fn pandoc_to_serde(data: &mut Value) {
    match *data {
        Value::Array(ref mut vec) => {
            for el in vec {
                pandoc_to_serde(el);
            }
        }
        Value::Object(ref mut map) => {
            if map.len() != 2 || !map.contains_key("c") || !map.contains_key("t") {
                for (_, v) in map {
                    pandoc_to_serde(v);
                }
                return
            }
            let t = map.remove("t").unwrap();
            if let Value::String(s) = t {
                let mut c = map.remove("c").unwrap();
                pandoc_to_serde(&mut c);
                map.insert(s, c);
            } else {
                unimplemented!()
            }
        }
        _ => {}
    }
}

pub fn filter<F: FnOnce(Pandoc)->Pandoc>(json: String, f: F) -> String {
    let mut data: Value = from_str(&json).unwrap();
    pandoc_to_serde(&mut data);
    let data = from_value(data).unwrap();
    let data = f(data);
    to_string(&data).unwrap()
}

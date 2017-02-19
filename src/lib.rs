extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod visitor;

pub use visitor::*;
pub use std::collections::BTreeMap as Map;
pub type Int = i64;
pub type Double = f64;

/// the root object of a pandoc document
#[derive(Serialize, Deserialize, Debug)]
pub struct Pandoc {
    pub meta: Map<String, MetaValue>,
    pub blocks: Vec<Block>,
    #[serde(rename="pandoc-api-version")]
    pub pandoc_api_version: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum MetaValue {
    MetaMap { c: Map<String, Box<MetaValue>> },
    MetaList { c: Vec<MetaValue> },
    MetaBool { c: bool },
    MetaString { c: String },
    MetaInlines { c: Vec<Inline> },
    MetaBlocks { c: Vec<Block> },
}

/// Structured text like tables and lists
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum Block {
    /// Plain text, not a paragraph
    Plain { c: Vec<Inline> },
    /// Paragraph
    Para { c: Vec<Inline> },
    /// Multiple non-breaking lines
    LineBlock { c: Vec<Vec<Inline>> },
    /// Code block (literal) with attributes
    CodeBlock { c: (Attr, String,) },
    RawBlock { c: (Format, String,) },
    /// Block quote (list of blocks)
    BlockQuote { c: Vec<Block> },
    /// Ordered list (attributes and a list of items, each a list of blocks)
    OrderedList { c: (ListAttributes, Vec<Vec<Block>>,) },
    /// Bullet list (list of items, each a list of blocks)
    BulletList { c: Vec<Vec<Block>> },
    /// Definition list Each list item is a pair consisting of a term (a list of inlines)
    /// and one or more definitions (each a list of blocks)
    DefinitionList { c: Vec<(Vec<Inline>, Vec<Vec<Block>>)> },
    /// Header - level (integer) and text (inlines)
    Header { c: (Int, Attr, Vec<Inline>,) },
    HorizontalRule,
    /// Table, with caption, column alignments (required), relative column widths (0 = default),
    /// column headers (each a list of blocks), and rows (each a list of lists of blocks)
    Table { c: (Vec<Inline>, Vec<Alignment>, Vec<Double>, Vec<TableCell>, Vec<Vec<TableCell>>,) },
    /// Generic block container with attributes
    Div { c: (Attr, Vec<Block>,) },
    /// Nothing
    Null,
}

/// a single formatting item like bold, italic or hyperlink
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum Inline {
    /// Text
    Str { c: String },
    /// Emphasized text
    Emph { c: Vec<Inline> },
    /// Strongly emphasized text
    Strong { c: Vec<Inline> },
    Strikeout { c: Vec<Inline> },
    Superscript { c: Vec<Inline> },
    Subscript { c: Vec<Inline> },
    SmallCaps { c: Vec<Inline> },
    /// Quoted text
    Quoted { c: (QuoteType,Vec<Inline>,) },
    /// Citation
    Cite { c: (Vec<Citation>, Vec<Inline>,) },
    /// Inline code (literal)
    Code { c: (Attr, String,) },
    /// Inter-word space
    Space,
    /// Soft line break
    SoftBreak,
    /// Hard line break
    LineBreak,
    /// TeX math (literal)
    Math { c: (MathType, String,) },
    RawInline { c: (Format, String,) },
    /// Hyperlink: text (list of inlines), target
    // "Link":[
    //    ["",[],[]],
    //    [{"Str":"1"}],
    //    ["#ref-scala_plugin",""]
    // ]

    Link { c: (Attr, Vec<Inline>, Target,) },
    /// Image: alt text (list of inlines), target
    Image { c: (Attr, Vec<Inline>, Target,) },
    /// Footnote or endnote
    Note { c: Vec<Block> },
    /// Generic inline container with attributes
    Span { c: (Attr, Vec<Inline>,) },
}

/// Alignment of a table column.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum Alignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
    AlignDefault,
}

pub type ListAttributes = (Int, ListNumberStyle, ListNumberDelim);

/// Style of list numbers.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum ListNumberStyle {
    DefaultStyle,
    Example,
    Decimal,
    LowerRoman,
    UpperRoman,
    LowerAlpha,
    UpperAlpha,
}

/// Delimiter of list numbers.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum ListNumberDelim {
    DefaultDelim,
    Period,
    OneParen,
    TwoParens,
}

/// Formats for raw blocks
#[derive(Serialize, Deserialize, Debug)]
pub struct Format(pub String);

/// Attributes: identifier, classes, key-value pairs
pub type Attr = (String, Vec<String>, Vec<(String, String)>);

/// Table cells are list of Blocks
pub type TableCell = Vec<Block>;

/// Type of quotation marks to use in Quoted inline.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum QuoteType {
    SingleQuote,
    DoubleQuote,
}

/// Link target (URL, title).
pub type Target = (String, String);

/// Type of math element (display or inline).
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum MathType {
    DisplayMath,
    InlineMath,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Citation {
    pub citationId: String,
    pub citationPrefix: Vec<Inline>,
    pub citationSuffix: Vec<Inline>,
    pub citationMode: CitationMode,
    pub citationNoteNum: Int,
    pub citationHash: Int,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    NormalCitation,
}

use serde_json::{from_str, to_string};

/// deserialized a json string to a Pandoc object, passes it to the closure/function
/// and serializes the result back into a string
pub fn filter<F: FnOnce(Pandoc)->Pandoc>(json: String, f: F) -> String {
    let v: serde_json::Value = from_str(&json).unwrap();
    let s = serde_json::to_string_pretty(&v).unwrap();
    println!("{}", s);
    let data: Pandoc = from_str(&s).expect("deserialization failed");
    assert_eq!(data.pandoc_api_version[0..2], [1, 17], "please file a bug report against `pandoc-ast` to update for the newest pandoc version");
    let data = f(data);
    to_string(&data).expect("serialization failed")
}

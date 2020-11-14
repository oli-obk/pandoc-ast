extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod visitor;

use serde_json::{from_str, to_string};

pub use std::collections::BTreeMap as Map;
pub use visitor::*;
pub type Int = i64;
pub type Double = f64;

/// the root object of a pandoc document
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Pandoc {
    pub meta: Map<String, MetaValue>,
    pub blocks: Vec<Block>,
    #[serde(rename = "pandoc-api-version")]
    pub pandoc_api_version: Vec<u32>,
}

impl Pandoc {
    fn from_json(json: &str) -> Self {
        let v: serde_json::Value = from_str(json).unwrap();
        let obj = v.as_object().expect("broken pandoc json");
        assert!(obj.contains_key("pandoc-api-version"), "Please update your pandoc to at least version 1.18 or use an older version of `pandoc-ast`");
        let s = serde_json::to_string_pretty(&v).unwrap();
        let data: Self = match from_str(&s) {
            Ok(data) => data,
            Err(err) => panic!("json is not in the pandoc format: {:?}\n{}", err, s),
        };
        //test major version
        assert_eq!(
            data.pandoc_api_version[0], 1,
            "pandoc-ast minor version mismatch: \
			 please file a bug report against `pandoc-ast` to update for the newest pandoc version"
        );
        
        // [1.21 , 1.22]
        assert!(
            (20..23).contains(&data.pandoc_api_version[1]),
            "pandoc-ast minor version mismatch: \
            please file a bug report against `pandoc-ast` to update for the newest pandoc version"
        );


        data
    }

    fn to_json(&self) -> String {
        to_string(self).expect("serialization failed")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum MetaValue {
    MetaMap(Map<String, Box<MetaValue>>),
    MetaList(Vec<MetaValue>),
    MetaBool(bool),
    MetaString(String),
    MetaInlines(Vec<Inline>),
    MetaBlocks(Vec<Block>),
}

/// Structured text like tables and lists
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum Block {
    /// Plain text, not a paragraph
    Plain(Vec<Inline>),
    /// Paragraph
    Para(Vec<Inline>),
    /// Multiple non-breaking lines
    LineBlock(Vec<Vec<Inline>>),
    /// Code block (literal) with attributes
    CodeBlock(Attr, String),
    RawBlock(Format, String),
    /// Block quote (list of blocks)
    BlockQuote(Vec<Block>),
    /// Ordered list (attributes and a list of items, each a list of blocks)
    OrderedList(ListAttributes, Vec<Vec<Block>>),
    /// Bullet list (list of items, each a list of blocks)
    BulletList(Vec<Vec<Block>>),
    /// Definition list Each list item is a pair consisting of a term (a list of inlines)
    /// and one or more definitions (each a list of blocks)
    DefinitionList(Vec<(Vec<Inline>, Vec<Vec<Block>>)>),
    /// Header - level (integer) and text (inlines)
    Header(Int, Attr, Vec<Inline>),
    HorizontalRule,
    /// Table, with attributes, caption, column alignments + widths
    /// column headers (each a list of rows), body and foot
    Table(
        Attr,
        Caption,
        Vec<ColSpec>,
        TableHead,
        Vec<TableBody>,
        TableFoot
    ),
    /// Generic block container with attributes
    Div(Attr, Vec<Block>),
    /// Nothing
    Null,
}

/// a single formatting item like bold, italic or hyperlink
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum Inline {
    /// Text
    Str(String),
    /// Emphasized text
    Emph(Vec<Inline>),
    /// Underlined text
    Underline(Vec<Inline>),
    /// Strongly emphasized text
    Strong(Vec<Inline>),
    Strikeout(Vec<Inline>),
    Superscript(Vec<Inline>),
    Subscript(Vec<Inline>),
    SmallCaps(Vec<Inline>),
    /// Quoted text
    Quoted(QuoteType, Vec<Inline>),
    /// Citation
    Cite(Vec<Citation>, Vec<Inline>),
    /// Inline code (literal)
    Code(Attr, String),
    /// Inter-word space
    Space,
    /// Soft line break
    SoftBreak,
    /// Hard line break
    LineBreak,
    /// TeX math (literal)
    Math(MathType, String),
    RawInline(Format, String),
    /// Hyperlink: text (list of inlines), target
    // "Link":[
    //    ["",[],[]],
    //    [{"Str":"1"}],
    //    ["#ref-scala_plugin",""]
    // ]
    Link(Attr, Vec<Inline>, Target),
    /// Image: alt text (list of inlines), target
    Image(Attr, Vec<Inline>, Target),
    /// Footnote or endnote
    Note(Vec<Block>),
    /// Generic inline container with attributes
    Span(Attr, Vec<Inline>),
}

/// Alignment of a table column.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(tag = "t")]
pub enum Alignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
    AlignDefault,
}

pub type ListAttributes = (Int, ListNumberStyle, ListNumberDelim);

/// Style of list numbers.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(tag = "t")]
pub enum ListNumberDelim {
    DefaultDelim,
    Period,
    OneParen,
    TwoParens,
}

/// Formats for raw blocks
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Format(pub String);

/// Attributes: identifier, classes, key-value pairs
pub type Attr = (String, Vec<String>, Vec<(String, String)>);

/// Table cells are list of Blocks
pub type TableCell = Vec<Block>;

/// Type of quotation marks to use in Quoted inline.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(tag = "t")]
pub enum QuoteType {
    SingleQuote,
    DoubleQuote,
}

/// Caption of a Table (Short caption, Caption)
pub type Caption = (Option<ShortCaption>, Vec<Block>);

/// Short caption of a Table
pub type ShortCaption = Vec<Inline>;

pub type RowHeadColumns = Int;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(tag = "t")]
pub enum ColWidth {
    ColWidth(Double),
    ColWidthDefault
}

pub type ColSpec = (Alignment, ColWidth);

pub type Row = (Attr, Vec<Cell>);

pub type TableHead = (Attr, Vec<Row>);

pub type TableBody = (Attr, RowHeadColumns, Vec<Row>, Vec<Row>);

pub type TableFoot = (Attr, Vec<Row>);

pub type Cell = (Attr, Alignment, RowSpan, ColSpan, Vec<Block>);

pub type RowSpan = Int;

pub type ColSpan = Int;
/// Link target (URL, title).
pub type Target = (String, String);

/// Type of math element (display or inline).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(tag = "t")]
pub enum MathType {
    DisplayMath,
    InlineMath,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct Citation {
    pub citationId: String,
    pub citationPrefix: Vec<Inline>,
    pub citationSuffix: Vec<Inline>,
    pub citationMode: CitationMode,
    pub citationNoteNum: Int,
    pub citationHash: Int,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(tag = "t")]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    NormalCitation,
}

/// deserialized a json string to a Pandoc object, passes it to the closure/function
/// and serializes the result back into a string
pub fn filter<F: FnOnce(Pandoc) -> Pandoc>(json: String, f: F) -> String {
    f(Pandoc::from_json(&json)).to_json()
}

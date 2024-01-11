mod visitor;

use serde_derive::{Deserialize, Serialize};
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
    pub fn from_json(json: &str) -> Self {
        let v: serde_json::Value = from_str(json).unwrap();
        let obj = v.as_object().expect("broken pandoc json");
        fn pandoc_version(obj: &serde_json::Map<String, serde_json::Value>) -> Option<(i64, i64)> {
            let version = obj
                .get("pandoc-api-version")?
                .as_array()?
                .iter()
                .map(|v| v.as_i64())
                .collect::<Vec<_>>();
            match version[..] {
                [Some(major), Some(minor), ..] => Some((major, minor)),
                _ => None,
            }
        }
        // test pandoc version
        const REQUIRED_PANDOC_VERSION: &str = "2.8";
        if let Some((major, minor)) = pandoc_version(obj) {
            let (required_major, required_minor) = (1, 20);
            if !(major == required_major && minor >= required_minor) {
                panic!(
                    "Pandoc version mismatch: \
                    `pandoc-ast` expects Pandoc AST version {}.{} or newer \
                    (`pandoc` {} or newer), got {}.{}",
                    required_major, required_minor, REQUIRED_PANDOC_VERSION, major, minor
                );
            }
        } else {
            panic!(
                "Unable to parse Pandoc AST version from JSON. \
                Please update your pandoc to at least version {} \
                or use an older version of `pandoc-ast`",
                REQUIRED_PANDOC_VERSION
            );
        }
        let s = serde_json::to_string_pretty(&v).unwrap();
        let data: Self = match from_str(&s) {
            Ok(data) => data,
            Err(err) => panic!("json is not in the pandoc format: {:?}\n{}", err, s),
        };
        data
    }

    pub fn to_json(&self) -> String {
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
    /// Figure, with attributes, caption, list of blocks
    Figure(Attr, Caption, Vec<Block>),
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
        TableFoot,
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
#[serde(tag = "t", content = "c")]
pub enum ColWidth {
    ColWidth(Double),
    ColWidthDefault,
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

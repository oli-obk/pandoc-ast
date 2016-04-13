extern crate pandoc_ast;
extern crate serde_json;

use pandoc_ast::*;

#[test]
fn format() {
    let s = r#""hello""#;
    let format: Format = serde_json::from_str(s).unwrap();
    assert_eq!(format.0, "hello");
}

#[test]
fn block() {
    let s = r#"{"Para":[{"RawInline":["tex","\\cake"]}]}"#;
    let block: Block = serde_json::from_str(s).unwrap();
    println!("{:#?}", block);
    let para = match block {
        Block::Para(para) => para,
        _ => panic!("not a para"),
    };
    assert_eq!(para.len(), 1);
    match para.into_iter().next().unwrap() {
        Inline::RawInline(format, text) => {
            assert_eq!(format.0, "tex");
            assert_eq!(text, "\\cake");
        },
        _ => panic!("not a rawinline"),
    }
}

#[test]
fn citation() {
    let s = r###"[{"unMeta":{}},[{"Para":[{"Cite":[[{"citationHash":1,"citationId":"scala_plugin","citationMode":{"NormalCitation":[]},"citationNoteNum":0,"citationPrefix":[],"citationSuffix":[]}],[{"Link":[["",[],[]],[{"Str":"1"}],["#ref-scala_plugin",""]]}]]}]}]]"###;
    filter(s.to_string(), |x| x);
}

#[test]
fn full_citation() {
    let s = r###"[{"unMeta":{"bibliography":{"MetaString":"bibliography.bib"},"csl":{"MetaString":"springer-basic-brackets-no-et-al-alphabetical.csl"},"link-citations":{"MetaBool":true}}},[{"Para":[{"Cite":[[{"citationHash":1,"citationId":"scala_plugin","citationMode":{"NormalCitation":[]},"citationNoteNum":0,"citationPrefix":[],"citationSuffix":[]}],[{"Str":"["},{"Link":[["",[],[]],[{"Str":"1"}],["#ref-scala_plugin",""]]},{"Str":"]"}]]}]},{"Header":[1,["literatur",["unnumbered"],[]],[{"Str":"Literatur"}]]},{"Para":[{"Str":" "},{"LineBreak":[]}]},{"Div":[["refs",["references"],[]],[{"Div":[["ref-scala_plugin",[],[]],[{"Para":[{"Str":"1"},{"Link":[["",[],[]],[{"Str":"text"}],["addr",""]]},{"Str":"."},{"Space":[]}]}]]}]]}]]"###;
    filter(s.to_string(), |x| x);
}

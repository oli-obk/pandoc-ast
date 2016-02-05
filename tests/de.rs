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

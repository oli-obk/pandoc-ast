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
/*
#[test]
fn citation() {
    let s = r###"[{"unMeta":{}},[{"Para":[{"Cite":[[{"citationHash":1,"citationId":"scala_plugin","citationMode":{"NormalCitation":[]},"citationNoteNum":0,"citationPrefix":[],"citationSuffix":[]}],[{"Link":[["",[],[]],[{"Str":"1"}],["#ref-scala_plugin",""]]}]]}]}]]"###;
    filter(s.to_string(), |x| x);
}

#[test]
fn full_citation() {
    let s = r###"[{"unMeta":{"bibliography":{"MetaString":"bibliography.bib"},"csl":{"MetaString":"springer-basic-brackets-no-et-al-alphabetical.csl"},"link-citations":{"MetaBool":true}}},[{"Para":[{"Cite":[[{"citationHash":1,"citationId":"scala_plugin","citationMode":{"NormalCitation":[]},"citationNoteNum":0,"citationPrefix":[],"citationSuffix":[]}],[{"Str":"["},{"Link":[["",[],[]],[{"Str":"1"}],["#ref-scala_plugin",""]]},{"Str":"]"}]]}]},{"Header":[1,["literatur",["unnumbered"],[]],[{"Str":"Literatur"}]]},{"Para":[{"Str":" "},{"LineBreak":[]}]},{"Div":[["refs",["references"],[]],[{"Div":[["ref-scala_plugin",[],[]],[{"Para":[{"Str":"1"},{"Link":[["",[],[]],[{"Str":"text"}],["addr",""]]},{"Str":"."},{"Space":[]}]}]]}]]}]]"###;
    filter(s.to_string(), |x| x);
}*/
/*
#[test]
fn image() {
    //let s = r####"[{"unMeta":{"date":{"t":"MetaInlines","c":[{"t":"Str","c":"Dr.-Ing."},{"t":"Space","c":[]},{"t":"Str","c":"Jörg"},{"t":"Space","c":[]},{"t":"Str","c":"Matthes"},{"t":"LineBreak","c":[]},{"t":"Str","c":"Dipl.-Inf."},{"t":"Space","c":[]},{"t":"Str","c":"Oliver"},{"t":"Space","c":[]},{"t":"Str","c":"Schneider"}]},"author":{"t":"MetaList","c":[{"t":"MetaInlines","c":[{"t":"Str","c":"Einführung"},{"t":"Space","c":[]},{"t":"Str","c":"C"}]}]},"title":{"t":"MetaInlines","c":[{"t":"Str","c":"Grundlagen"},{"t":"Space","c":[]},{"t":"Str","c":"der"},{"t":"Space","c":[]},{"t":"Str","c":"Informatik"},{"t":"LineBreak","c":[]},{"t":"Str","c":"Teil"},{"t":"Space","c":[]},{"t":"Str","c":"1"}]}}},[{"t":"Header","c":[1,["grundlagen",[],[]],[{"t":"Str","c":"Grundlagen"}]]},{"t":"Header","c":[2,["einführung-programmiersprachen",[],[]],[{"t":"Str","c":"Einführung"},{"t":"Space","c":[]},{"t":"Str","c":"Programmiersprachen"}]]},{"t":"Para","c":[{"t":"Image","c":[[],["ProgrammiersprachenVerwandschaft.png","fig:"]]}]},{"t":"Header","c":[1,["baaa",[],[]],[{"t":"Str","c":"BAAA"}]]},{"t":"BulletList","c":[[{"t":"Plain","c":[{"t":"Str","c":"bee"}]}],[{"t":"Plain","c":[{"t":"Str","c":"boo"}]}]]},{"t":"Header","c":[1,["bool",[],[]],[{"t":"Str","c":"Bool"}]]},{"t":"Para","c":[{"t":"Str","c":"bar"}]}]]"####;
    let s = r####"{"t":"Image","c":[[],["ProgrammiersprachenVerwandschaft.png","fig:"]]}"####;
    let mut value: serde_json::Value = serde_json::from_str(s).unwrap();
    println!("{:?}", value);
    pandoc_to_serde(&mut value);
    println!("{:?}", value);
    let _: Inline = serde_json::from_value(value).unwrap();
}
*/

#[test]
fn one_point_seven() {
    let s = r####"{"pandoc-api-version":[1,17,0,4],"meta":{"title":{"t":"MetaInlines","c":[{"t":"Str","c":"Grundlagen"},{"t":"Space"},{"t":"Str","c":"der"},{"t":"Space"},{"t":"Str","c":"Informatik"},{"t":"RawInline","c":["tex","\\"]}]}},"blocks":[{"t":"Para","c":[{"t":"Str","c":"Teil"},{"t":"Space"},{"t":"Str","c":"1"},{"t":"SoftBreak"},{"t":"Str","c":"%"},{"t":"Space"},{"t":"Str","c":"Einführung"},{"t":"Space"},{"t":"Str","c":"C"},{"t":"SoftBreak"},{"t":"Str","c":"%"},{"t":"Space"},{"t":"Str","c":"Dr.-Ing."},{"t":"Space"},{"t":"Str","c":"Jörg"},{"t":"Space"},{"t":"Str","c":"Matthes"},{"t":"LineBreak"},{"t":"Str","c":"Dipl.-Inf."},{"t":"Space"},{"t":"Str","c":"Oliver"},{"t":"Space"},{"t":"Str","c":"Schneider"}]},{"t":"Header","c":[1,["grundlagen",[],[]],[{"t":"Str","c":"Grundlagen"}]]},{"t":"Header","c":[2,["programmiersprachen",[],[]],[{"t":"Str","c":"Programmiersprachen"}]]},{"t":"Para","c":[{"t":"Str","c":"ProgrammiersprachenVerwandschaft.png"}]},{"t":"Header","c":[2,["programmiersprachen-c-und-c",[],[]],[{"t":"Str","c":"Programmiersprachen"},{"t":"Space"},{"t":"Str","c":"C"},{"t":"Space"},{"t":"Str","c":"und"},{"t":"Space"},{"t":"Str","c":"C++"}]]},{"t":"Header","c":[3,["c",[],[]],[{"t":"Str","c":"C"}]]},{"t":"Para","c":[{"t":"Str","c":"imperative"},{"t":"Space"},{"t":"Str","c":"(befehlsorientierte)"},{"t":"Space"},{"t":"Str","c":"Programmiersprache"}]},{"t":"Header","c":[3,["c-1",[],[]],[{"t":"Str","c":"C++"}]]},{"t":"BulletList","c":[[{"t":"Plain","c":[{"t":"Str","c":"Weiterentwicklung"},{"t":"Space"},{"t":"Str","c":"von"},{"t":"Space"},{"t":"Str","c":"C"}]}],[{"t":"Plain","c":[{"t":"Str","c":"objektorientiert"}]}],[{"t":"Plain","c":[{"t":"Str","c":"entwickelt"},{"t":"Space"},{"t":"Str","c":"von"},{"t":"Space"},{"t":"Str","c":"Bjarne"},{"t":"Space"},{"t":"Str","c":"Stroustrup"}]}]]},{"t":"Para","c":[{"t":"Str","c":"Stroustrup.png"}]},{"t":"Header","c":[2,["literaturhinweis",[],[]],[{"t":"Str","c":"Literaturhinweis"}]]},{"t":"Para","c":[{"t":"Str","c":"*..."}]}]}"####;
    let mut value: serde_json::Value = serde_json::from_str(s).unwrap();
    pandoc_to_serde(&mut value);
    println!("{:?}", value);
    let _: Pandoc = serde_json::from_value(value).unwrap();
}

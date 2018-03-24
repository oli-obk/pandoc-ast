extern crate pandoc_ast;

use pandoc_ast::{MutVisitor, Inline};
use std::io::{self, Write, Read};

struct MyVisitor;

impl MutVisitor for MyVisitor {
    fn visit_vec_inline(&mut self, vec_inline: &mut Vec<Inline>) {
        vec_inline.retain(|inline| match inline {
            &Inline::Note(_) => false,
            _ => true
        });
        self.walk_vec_inline(vec_inline);
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let s = pandoc_ast::filter(s, |mut pandoc| {
        MyVisitor.walk_pandoc(&mut pandoc);
        pandoc
    });
    io::stdout().write(s.as_bytes()).unwrap();
}

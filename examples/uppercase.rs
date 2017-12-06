extern crate pandoc_ast;

use pandoc_ast::{MutVisitor, Inline};
use std::io::{self, Write, Read};

struct MyVisitor;

impl MutVisitor for MyVisitor {
    fn visit_inline(&mut self, inline: &mut Inline) {
        if let Inline::Str(ref mut s) = *inline {
            *s = s.to_uppercase();
            return;
        }
        self.walk_inline(inline);
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

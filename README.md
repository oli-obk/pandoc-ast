[![Build Status](https://travis-ci.org/oli-obk/pandoc-ast.svg?branch=master)](https://travis-ci.org/oli-obk/pandoc-ast)

# Instructions

Use in conjunction with the `pandoc` crate.

```rust
extern crate pandoc;
extern crate pandoc_ast;

fn main() {
    let mut pandoc = pandoc::new();

    ...

    pandoc.add_filter(|json| pandoc_ast::filter(json, |mut pandoc| {
        for block in &mut pandoc.1 {
            use pandoc_ast::Block::*;
            *block = match *block {
                CodeBlock((_, ref kinds, _), _) if kinds.iter().next() == Some("graphviz") => {
                    // do something to change a graphviz block into an image
                }
            }
        }
        pandoc
    }));
    pandoc.execute().unwrap();
}
```

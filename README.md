This crate allows you to implement filters for pandoc.
The easiest way is to them in conjunction with the `pandoc` crate.
You can also create a binary that reads from stdin and writes to stdout and
pass that to a normal pandoc call with `--filter`

# Instructions

```rust
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

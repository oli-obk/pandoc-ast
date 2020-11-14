use super::*;

pub trait MutVisitor {
    fn visit_block(&mut self, block: &mut Block) {
        self.walk_block(block)
    }
    fn visit_attr(&mut self, attr: &mut Attr) {
        self.walk_attr(attr)
    }
    fn visit_inline(&mut self, inline: &mut Inline) {
        self.walk_inline(inline)
    }
    fn visit_meta(&mut self, _key: &str, meta: &mut MetaValue) {
        self.walk_meta(meta)
    }
    fn visit_vec_block(&mut self, vec_block: &mut Vec<Block>) {
        self.walk_vec_block(vec_block)
    }
    fn visit_vec_inline(&mut self, vec_inline: &mut Vec<Inline>) {
        self.walk_vec_inline(vec_inline)
    }
    fn visit_rows(&mut self, rows: &mut Vec<Row>) {
        self.walk_rows(rows)
    }
    fn walk_meta(&mut self, meta: &mut MetaValue) {
        use MetaValue::*;
        match *meta {
            MetaMap(ref mut c) => {
                for (key, meta) in c {
                    self.visit_meta(key, meta);
                }
            }
            MetaList(ref mut c) => {
                for meta in c {
                    self.walk_meta(meta);
                }
            }
            MetaBool(_) => {}
            MetaString(_) => {}
            MetaInlines(ref mut v_inline) => {
                self.visit_vec_inline(v_inline);
            }
            MetaBlocks(ref mut v_block) => {
                self.visit_vec_block(v_block);
            }
        }
    }
    fn walk_pandoc(&mut self, pandoc: &mut Pandoc) {
        for (key, meta) in &mut pandoc.meta {
            self.visit_meta(key, meta);
        }
        self.visit_vec_block(&mut pandoc.blocks);
    }
    fn walk_block(&mut self, block: &mut Block) {
        use Block::*;
        match *block {
            Plain(ref mut vec_inline) | Para(ref mut vec_inline) => {
                self.visit_vec_inline(vec_inline);
            }
            LineBlock(ref mut vec_vec_inline) => {
                for vec_inline in vec_vec_inline {
                    self.visit_vec_inline(vec_inline);
                }
            }
            CodeBlock(ref mut attr, _) => self.visit_attr(attr),
            RawBlock { .. } => {}
            BlockQuote(ref mut vec_block) => {
                self.visit_vec_block(vec_block);
            }
            OrderedList(_, ref mut vec_vec_block) | BulletList(ref mut vec_vec_block) => {
                for vec_block in vec_vec_block {
                    self.visit_vec_block(vec_block);
                }
            }
            DefinitionList(ref mut c) => {
                for def in c {
                    self.visit_vec_inline(&mut def.0);
                    for vec_block in &mut def.1 {
                        self.visit_vec_block(vec_block);
                    }
                }
            }
            Header(_, ref mut attr, ref mut vec_inline) => {
                self.visit_attr(attr);
                self.visit_vec_inline(vec_inline);
            }
            HorizontalRule => {}
            Table(ref mut attr, ref mut caption, _, ref mut head, ref mut bodies, ref mut foot) => {
                self.visit_attr(attr);
                {
                    let (short, caption) = caption;
                    if let Some(shortcaption) = short {
                        self.visit_vec_inline(shortcaption);
                    }

                    self.visit_vec_block(caption);
                }
                {
                    let (attr, rows) = head;
                    self.visit_attr(attr);
                    self.visit_rows(rows);
                }
                for body in bodies {
                    let (attr, _, rows_h, rows) = body;
                    self.visit_attr(attr);
                    self.visit_rows(rows_h);
                    self.visit_rows(rows);

                }
                {
                    let (attr, rows) = foot;
                    self.visit_attr(attr);
                    self.visit_rows(rows);
                }

            }
            Div(ref mut attr, ref mut vec_block) => {
                self.visit_attr(attr);
                self.visit_vec_block(vec_block);
            }
            Null => {}
        }
    }
    fn walk_attr(&mut self, _attr: &mut Attr) {}
    fn walk_inline(&mut self, inline: &mut Inline) {
        use Inline::*;
        match *inline {
            Str { .. } => {}
            Emph(ref mut c)
            | Strong(ref mut c)
            | Underline(ref mut c)
            | Strikeout(ref mut c)
            | Superscript(ref mut c)
            | Subscript(ref mut c)
            | SmallCaps(ref mut c)
            | Quoted(_, ref mut c) => {
                self.visit_vec_inline(c);
            }
            Cite(ref mut v_cite, ref mut v_inl) => {
                for cite in v_cite {
                    self.visit_vec_inline(&mut cite.citationPrefix);
                    self.visit_vec_inline(&mut cite.citationSuffix);
                }
                self.visit_vec_inline(v_inl);
            }
            Code(ref mut attr, _) => self.visit_attr(attr),
            Space { .. } => {}
            SoftBreak { .. } => {}
            LineBreak { .. } => {}
            Math { .. } => {}
            RawInline { .. } => {}
            Link(ref mut attr, ref mut v_inline, _)
            | Image(ref mut attr, ref mut v_inline, _)
            | Span(ref mut attr, ref mut v_inline) => {
                self.visit_attr(attr);
                self.visit_vec_inline(v_inline);
            }
            Note(ref mut c) => {
                self.visit_vec_block(c);
            }
        }
    }
    fn walk_rows(&mut self, rows: &mut Vec<Row>){
        for (attr, cells) in rows {
            self.visit_attr(attr);
            for (cell_attr, _, _, _, content) in cells {
                self.visit_attr(cell_attr);
                self.visit_vec_block(content);
            }
        }
    }
    fn walk_vec_block(&mut self, vec_block: &mut Vec<Block>) {
        for block in vec_block {
            self.visit_block(block);
        }
    }
    fn walk_vec_inline(&mut self, vec_inline: &mut Vec<Inline>) {
        for inline in vec_inline {
            self.visit_inline(inline);
        }
    }
}

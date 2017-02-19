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
    fn walk_meta(&mut self, meta: &mut MetaValue) {
        use MetaValue::*;
        match *meta {
            MetaMap { ref mut c } => for (key, meta) in c {
                self.visit_meta(key, meta);
            },
            MetaList{ ref mut c } => for meta in c {
                self.walk_meta(meta);
            },
            MetaBool { .. } => {},
            MetaString { .. } => {},
            MetaInlines{ c: ref mut v_inline} => for inline in v_inline {
                self.visit_inline(inline);
            },
            MetaBlocks{ c: ref mut v_block} => for block in v_block {
                self.visit_block(block);
            },
        }
    }
    fn walk_pandoc(&mut self, pandoc: &mut Pandoc) {
        for (key, meta) in &mut pandoc.meta {
            self.visit_meta(key, meta);
        }
        for block in &mut pandoc.blocks {
            self.visit_block(block);
        }
    }
    fn walk_block(&mut self, block: &mut Block) {
        use Block::*;
        match *block {
            Plain { c: ref mut vec_inline } |
            Para { c: ref mut vec_inline } => for inline in vec_inline {
                self.visit_inline(inline);
            },
            LineBlock { c: ref mut vec_vec_inline} => for vec_inline in vec_vec_inline {
                for inline in vec_inline {
                    self.visit_inline(inline);
                }
            },
            CodeBlock{ c: (ref mut attr, _) } => self.visit_attr(attr),
            RawBlock{..} => {},
            BlockQuote{ c: ref mut vec_block } => for block in vec_block {
                self.visit_block(block);
            },
            OrderedList{ c: (_, ref mut vec_vec_block) } |
            BulletList{ c: ref mut vec_vec_block } => for vec_block in vec_vec_block {
                for block in vec_block {
                    self.visit_block(block);
                }
            },
            DefinitionList{ ref mut c } => for def in c {
                for inline in &mut def.0 {
                    self.visit_inline(inline);
                }
                for vec_block in &mut def.1 {
                    for block in vec_block {
                        self.visit_block(block);
                    }
                }
            },
            Header { c: (_, ref mut attr, ref mut vec_inline) } => {
                self.visit_attr(attr);
                for inline in vec_inline {
                    self.visit_inline(inline);
                }
            },
            HorizontalRule => {},
            Table { c: (ref mut vec_inline, _, _, ref mut vv_block, ref mut vvv_block) } => {
                for inline in vec_inline {
                    self.visit_inline(inline);
                }
                for vec_block in vv_block {
                    for block in vec_block {
                        self.visit_block(block);
                    }
                }
                for vv_block in vvv_block {
                    for vec_block in vv_block {
                        for block in vec_block {
                            self.visit_block(block);
                        }
                    }
                }
            },
            Div { c: (ref mut attr, ref mut vec_block) } => {
                self.visit_attr(attr);
                for block in vec_block {
                    self.visit_block(block);
                }
            },
            Null => {},
        }
    }
    fn walk_attr(&mut self, _attr: &mut Attr) {}
    fn walk_inline(&mut self, inline: &mut Inline) {
        use Inline::*;
        match *inline {
            Str{..} => {},
            Emph{ ref mut c } |
            Strong{ ref mut c } |
            Strikeout{ ref mut c } |
            Superscript{ ref mut c } |
            Subscript{ ref mut c } |
            SmallCaps{ ref mut c } |
            Quoted { c: (_, ref mut c,) } => for inline in c {
                self.visit_inline(inline);
            },
            Cite { c: (ref mut v_cite, ref mut v_inl) } => {
                for cite in v_cite {
                    for inline in &mut cite.citationPrefix {
                        self.visit_inline(inline);
                    }
                    for inline in &mut cite.citationSuffix {
                        self.visit_inline(inline);
                    }
                }
                for inline in v_inl {
                    self.visit_inline(inline);
                }
            },
            Code { c: (ref mut attr, _) } => self.visit_attr(attr),
            Space{..}  => {},
            SoftBreak{..} => {},
            LineBreak{..} => {},
            Math {..} => {},
            RawInline{..} => {},
            Link { c: (ref mut attr, ref mut v_inline, _) } |
            Image { c: (ref mut attr, ref mut v_inline, _) } |
            Span { c: (ref mut attr, ref mut v_inline) } => {
                self.visit_attr(attr);
                for inline in v_inline {
                    self.visit_inline(inline);
                }
            },
            Note { ref mut c } => for block in c {
                self.visit_block(block);
            },
        }
    }
}

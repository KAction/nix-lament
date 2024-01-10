use crate::types;
use anyhow;
use tree_sitter as ts;
use tree_sitter_nix as nix;

pub fn node_text<'a>(node: &ts::Node, content: &'a [u8]) -> &'a str {
    std::str::from_utf8(&content[node.start_byte()..node.end_byte()]).unwrap()
}

type Handler = fn(m: &ts::QueryMatch, content: &[u8]) -> Option<types::Lamentation>;
pub struct PerMatch {
    query: ts::Query,
    handler: Handler,
}

impl PerMatch {
    pub fn new(query: &'static str, handler: Handler) -> anyhow::Result<Box<dyn types::Lament>> {
        Ok(Box::new(Self {
            query: ts::Query::new(nix::language(), query)?,
            handler,
        }))
    }
}

impl types::Lament for PerMatch {
    fn lament(&self, tree: &ts::Tree, content: &[u8]) -> Vec<types::Lamentation> {
        let mut out = vec![];
        let mut cursor = ts::QueryCursor::new();

        for m in cursor.matches(&self.query, tree.root_node(), content) {
            match (self.handler)(&m, content) {
                Some(x) => out.push(x),
                None => (),
            }
        }
        out
    }
}

#[macro_export]
macro_rules! via_match (
    ($name:ident, $handler:expr) => {
        fn new() -> anyhow::Result<Box<dyn crate::types::Lament>> {
            crate::util::PerMatch::new(include_str!(concat!(stringify!($name), ".scm")), $handler)
        }
        pub static MODULE: crate::types::Module = crate::types::Module {
            new, kinds: &[crate::types::Kind::$name],
        };
    };
);

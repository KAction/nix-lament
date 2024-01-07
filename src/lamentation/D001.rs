use crate::types;
use anyhow;
use tree_sitter as ts;
use tree_sitter_nix as nix;

struct I {
    query: ts::Query,
}

fn new() -> anyhow::Result<Box<dyn types::Lament>> {
    Ok(Box::new(I {
        query: ts::Query::new(nix::language(), include_str!("D001.scm"))?,
    }))
}

fn node_text<'a>(node: &ts::Node, content: &'a [u8]) -> &'a str {
    std::str::from_utf8(&content[node.start_byte()..node.end_byte()]).unwrap()
}

impl types::Lament for I {
    fn lament(&self, tree: &ts::Tree, content: &[u8]) -> Vec<types::Lamentation> {
        let mut out = vec![];
        let mut cursor = ts::QueryCursor::new();

        for m in cursor.matches(&self.query, tree.root_node(), content) {
            let func = node_text(&m.captures[0].node, content);
            let n2 = m.captures[2]; // second name|pname match
            let point = n2.node.start_position();
            let message = format!("Call to `{}' has both `name' and `pname' arguments.", func);

            out.push(types::Lamentation {
                kind: types::Kind::D001,
                line: point.row,
                column: point.column,
                message,
            })
        }
        out
    }
}

pub static MODULE: types::Module = types::Module {
    kinds: &[types::Kind::D001],
    new,
};

#[test]
fn test_lament_D001() {
    let content = include_bytes!("../../t/D001.nix");
    let mut parser = ts::Parser::new();
    parser.set_language(nix::language()).unwrap();

    let tree = parser.parse(&content, None).unwrap();
    let w = new().unwrap();

    insta::assert_debug_snapshot!(w.lament(&tree, &content[..]));
}

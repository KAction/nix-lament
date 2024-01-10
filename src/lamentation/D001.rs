use crate::types;
use crate::util::node_text;
use crate::via_match;

via_match!(D001, |m, content| {
    let func = node_text(&m.captures[0].node, content);
    let n2 = m.captures[2]; // second name|pname match
    let point = n2.node.start_position();
    let message = format!("Call to `{}' has both `name' and `pname' arguments.", func);

    Some(types::Lamentation {
        kind: types::Kind::D001,
        line: point.row,
        column: point.column,
        message,
    })
});

#[test]
fn test_lament_D001() {
    use tree_sitter as ts;
    use tree_sitter_nix as nix;

    let content = include_bytes!("../../t/D001.nix");
    let mut parser = ts::Parser::new();
    parser.set_language(nix::language()).unwrap();

    let tree = parser.parse(&content, None).unwrap();
    let w = new().unwrap();

    insta::assert_debug_snapshot!(w.lament(&tree, &content[..]));
}

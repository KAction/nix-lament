use crate::types;
use crate::util::node_text;
use crate::via_match;

via_match!(PY001, |m, content| {
    let func = node_text(&m.captures[0].node, content);
    let n2 = m.captures[2]; // pythonImportsCheckHook
    let point = n2.node.start_position();
    // nixos/nixpkgs#279667
    let message = format!(
        "Explicit `pythonImportsCheckHook` in `nativeBuildInputs` of `{}' call.",
        func
    );

    Some(types::Lamentation {
        kind: types::Kind::PY001,
        line: point.row,
        column: point.column,
        message,
    })
});

#[test]
fn test_lament_D001() {
    use tree_sitter as ts;
    use tree_sitter_nix as nix;

    let content = include_bytes!("../../t/PY001_t01.nix");
    let mut parser = ts::Parser::new();
    parser.set_language(nix::language()).unwrap();

    let tree = parser.parse(&content, None).unwrap();
    let w = new().unwrap();

    insta::assert_debug_snapshot!(w.lament(&tree, &content[..]));
}

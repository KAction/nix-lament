use tree_sitter as ts;

pub fn node_text<'a>(node: &ts::Node, content: &'a [u8]) -> &'a str {
    std::str::from_utf8(&content[node.start_byte()..node.end_byte()]).unwrap()
}

#[macro_export]
macro_rules! via_match (
    ($name:ident, $handler:expr) => {
        fn new() -> anyhow::Result<crate::types::PerMatch> {
            crate::types::PerMatch::new(include_str!(concat!(stringify!($name), ".scm")), $handler)
        }
        pub static MODULE: crate::types::Module = crate::types::Module {
            new, kinds: &[crate::types::Kind::$name],
        };

        #[test]
        fn test_snapshots() {
            use tree_sitter as ts;
            use tree_sitter_nix as nix;
            use glob::glob;

            let mut parser = ts::Parser::new();
            parser.set_language(nix::language()).unwrap();

            let w = new().unwrap();
            let pattern = format!("t/{}/*.nix", stringify!($name));

            for entry in glob(&pattern).unwrap() {
                let path = entry.unwrap();
                let content = std::fs::read(&path).unwrap();
                let tree = parser.parse(&content, None).unwrap();
                let snapshot_name = path.as_os_str().to_str().unwrap();

                insta::assert_yaml_snapshot!(snapshot_name, w.lament(&tree, &content[..]));
                parser.reset();
            };
        }

    };
);

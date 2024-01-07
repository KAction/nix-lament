use tree_sitter as ts;
use tree_sitter_nix as nix;
use anyhow;

fn main() -> anyhow::Result<()>  {
    let language = nix::language();
    let mut parser = ts::Parser::new();

    parser.set_language(language)?;

    for fname in std::env::args().skip(1) {
        println!("Parsing {}", fname);
        let content = std::fs::read(fname)?;
        // None is only returned on misuse or timeout.
        let tree = parser.parse(&content, None).unwrap();
        let query = ts::Query::new(language, include_str!("D001.scm"))?;
        let mut cursor = ts::QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), &content[..]);
        let capture_names = query.capture_names();

        for m in matches {
            println!("match {}::", m.pattern_index);
            for c in m.captures {
                let node = c.node;
                let string = std::str::from_utf8(&content[node.start_byte()..node.end_byte()])?;

                println!("Name @{} captured node {} of kind '{}' and text `{}`", capture_names[c.index as usize], node.id(), node.kind(), string);
            }
        }

        parser.reset();
    }

    // let query = include_str!("D001.scm");
    // println!("{}", query);
    Ok(())
}

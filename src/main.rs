use anyhow;
use tree_sitter as ts;
use tree_sitter_nix as nix;

mod lamentation;
mod types;

fn main() -> anyhow::Result<()> {
    let language = nix::language();
    let mut parser = ts::Parser::new();
    parser.set_language(language)?;

    let mut workers = vec![];
    for x in lamentation::MODULES {
        workers.push((x.new)()?);
    }

    for fname in std::env::args().skip(1) {
        let content = std::fs::read(&fname)?;
        // None is only returned on misuse or timeout.
        let tree = parser.parse(&content, None).unwrap();

        for w in &workers {
            for e in w.lament(&tree, &content) {
                println!("{}:{}:{}: {:?} {}", &fname, e.line, e.column, e.kind, e.message);
            }
        }

        parser.reset();
    }
    Ok(())
}

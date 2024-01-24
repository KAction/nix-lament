use anyhow;
use serde::Serialize;
use tree_sitter as ts;
use tree_sitter_nix as nix;

/// Kind of the lamentations. This is necessary to run only requested subset of supported
/// lamentations.
#[derive(Debug, Serialize)]
pub enum Kind {
    D001,  // Both "pname" and "name" in a call to "mkDerivation"
    PY001, // Explicit "pythonImportsCheckHook" in nativeBuildInputs
    RFC0169, // Naming of the feature parameters
}

// Struct that describes location of the problematic code.
#[derive(Debug, Serialize)]
pub struct Lamentation {
    // By convention, lines are counted from one, columns -- from zero.
    pub line: usize,
    pub column: usize,
    pub kind: Kind,
    pub message: String,
}

pub type Handler = fn(m: &ts::QueryMatch, content: &[u8]) -> Option<Lamentation>;
pub struct PerMatch {
    query: ts::Query,
    handler: Handler,
}

impl PerMatch {
    pub fn new(query: &'static str, handler: Handler) -> anyhow::Result<Self> {
        Ok(Self {
            query: ts::Query::new(nix::language(), query)?,
            handler,
        })
    }

    pub fn lament(&self, tree: &ts::Tree, content: &[u8]) -> Vec<Lamentation> {
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

#[derive(Clone, Copy)]
pub struct Module {
    pub kinds: &'static [Kind],
    pub new: fn() -> anyhow::Result<PerMatch>,
}

use anyhow;
use tree_sitter as ts;

/// Kind of the lamentations. This is necessary to run only requested subset of supported
/// lamentations.
#[derive(Debug)]
pub enum Kind {
    D001, // Both "pname" and "name" in a call to "mkDerivation"
}

// Struct that describes location of the problematic code.
pub struct Lamentation {
    // By convention, lines are counted from one, columns -- from zero.
    pub line: usize,
    pub column: usize,
    pub kind: Kind,
    pub message: String,
}

// "lament" is a method, not plain function so we have space for
// once-per-module initialization (e.g parsing queries).
pub trait Lament {
    fn lament(&self, tree: &ts::Tree, content: &[u8]) -> Vec<Lamentation>;
}

#[derive(Clone, Copy)]
pub struct Module {
    pub kinds: &'static [Kind],
    pub new: fn() -> anyhow::Result<Box<dyn Lament>>,
}

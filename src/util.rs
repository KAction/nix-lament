use tree_sitter as ts;

pub fn node_text<'a>(node: &ts::Node, content: &'a [u8]) -> &'a str {
    std::str::from_utf8(&content[node.start_byte()..node.end_byte()]).unwrap()
}


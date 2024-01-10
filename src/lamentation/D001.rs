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

use crate::types;
use crate::util::node_text;
use crate::via_match;
use phf::phf_map;

static REPLACEMENTS: phf::Map<&'static str, &'static str> = phf_map! {
    "alsaSupport" => "with_alsa",
    "enableAlsa" => "with_alsa",
};

via_match!(RFC0169, |m, content| {
    let deprecated = &m.captures[1].node;
    let deprected_name = node_text(deprecated, content);
    let point = deprecated.start_position();
    let new_name = REPLACEMENTS.get(deprected_name).expect(&format!("Replacement for `{}' not found", deprected_name));
    let message = format!("Deprecated feature parameter `{}', use `{}` name instead", deprected_name, new_name);

    Some(types::Lamentation {
        kind: types::Kind::RFC0169,
        line: point.row,
        column: point.column,
        message,
    })
});

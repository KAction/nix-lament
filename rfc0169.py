import json
from itertools import chain
import sys

config = json.load(sys.stdin)
deprecated = "|".join(chain(*config.values()))
with open("src/lamentation/RFC0169.scm", "w") as fp:
    fp.write(f"""
(function_expression
  (formals
    (formal (identifier) @lib (#eq? @lib "lib"))
    (formal (identifier) @deprecated (#match? @deprecated "^({deprecated})$"))))
;; vim:ft=query:ro:
""")

with open("src/lamentation/RFC0169_autogen.rs", "w") as fp:
    fp.write("use phf::phf_map;\n")
    fp.write("pub static REPLACEMENTS: phf::Map<&'static str, &'static str> = phf_map! {\n")
    for new, deprecated in config.items():
        for old in deprecated:
            fp.write(f'\t"{old}" => "{new}",\n')
    fp.write("};")

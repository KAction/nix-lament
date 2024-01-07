(apply_expression
  (variable_expression name: (identifier) @name (#eq? @name "mkDerivation"))
  (attrset_expression
    (binding_set
      (binding (attrpath (identifier) @k1 (#match? @k1 "^(name|pname)$")))
      (binding (attrpath (identifier) @k2 (#match? @k2 "^(name|pname)$"))))))
; vim:ft=query:

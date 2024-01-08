(apply_expression
  (variable_expression
    (identifier) @func
    (#match? @func "^buildPython(Package|Application)$"))
  (attrset_expression
    (binding_set
      (binding
        (attrpath (identifier) @key (#eq? @key "nativeBuildInputs"))
        (list_expression
          (variable_expression (identifier) @input (#eq? @input "pythonImportsCheckHook")))))))
;; vim:ft=query:

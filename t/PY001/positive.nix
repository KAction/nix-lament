{ buildPythonPackage, pythonImportsCheckHook, sphinxHook }:

buildPythonPackage {
  pname = "foo";
  version = "1.0";

  nativeBuildInputs = [
    sphinxHook
    pythonImportsCheckHook
  ];
};

{ mkDerivation }:

mkDerivation {
  name = "foo";
  pname = "bar";

  bar = foo 10;
  baz = bar 52;

  buildPhase = ''
    make -C bar
  '';
}

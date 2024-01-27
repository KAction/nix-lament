{
  description = "Yet another nixpkgs linter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/cf28ee258fd5f9a52de6b9865cdb93a1f96d09b7";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

outputs = { self, nixpkgs, fenix, flake-utils, crane, ... }:
    let
      inherit (nixpkgs.lib) optionals;
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        fenixPkgs = fenix.packages.${system};
        pkgs = nixpkgs.legacyPackages.${system};

        toolchain = fenixPkgs.combine [
          fenixPkgs.stable.cargo
          fenixPkgs.stable.clippy
          fenixPkgs.stable.rustc
          fenixPkgs.stable.rustfmt
          fenixPkgs.stable.rust-std
          fenixPkgs.targets.x86_64-unknown-linux-musl.stable.rust-std
        ];
        craneLib = crane.lib.${system}.overrideToolchain toolchain;
      in
      let
        source = nixpkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = path: type:
            craneLib.filterCargoSources path type ||
            builtins.match ".*scm" path != null ||
            builtins.match ".*snap" path != null ||
            builtins.match ".*nix" path != null;
        };

        commonArgs = {
          src = source;
          buildInputs = [ ] ++ optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
        };
        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          pname = "nix-lament-deps";
        });
        cargoClippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });
        cargoPackage = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      rec {
        packages.default = cargoPackage;
        checks = { inherit cargoPackage cargoClippy; };

        devShells.default = pkgs.mkShell {
          packages = [
            toolchain
            fenixPkgs.rust-analyzer
            pkgs.cargo-expand
            pkgs.cargo-bloat
          ];

          buildInputs = optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.CoreServices
            pkgs.libiconv
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };

        devShells.msrv = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rustup
            pkgs.cargo-msrv
          ];
        };
      });
}

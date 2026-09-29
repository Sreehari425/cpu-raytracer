{
  description = "Rust dev env";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        cargoToml = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
      in
      {
        packages.default = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          inherit (cargoToml) pname version;

          buildInputs = [ ];
          nativeBuildInputs = [ ];
        };

        devShells.default = pkgs.mkShell {
          name = "\${cargoToml.pname}-dev-shell";

          nativeBuildInputs = [
            rustToolchain
            pkgs.pkg-config
          ];

          RUST_SRC_PATH = "\${rustToolchain}/lib/rustlib/src/rust/library";

          shellHook = ''
            echo "in dev env btw :)"
            echo "Using \$(rustc --version)"

          '';
        };
      }
    );
}

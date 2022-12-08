{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/release-22.05;
    utils.url = github:numtide/flake-utils;

    rust-overlay = {
      url = github:oxalica/rust-overlay;
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "utils";
      };
    };
  };

  outputs = inputs@{ nixpkgs, utils, rust-overlay, ... }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
          rust = pkgs.rust-bin.stable."1.65.0";
          # rust = pkgs.rust-bin.nightly.latest;
          shell = pkgs.mkShell {
            buildInputs = [
              pkgs.pkg-config
              
              (rust.default.override { extensions = [ "rust-analyzer" "rust-src" ]; })
            ];
          };
       in
      {
        devShells.default = shell;
      });
}
{
  description = "Flake for jellyfin-rpc";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in {
      packages.default = pkgs.callPackage ./default.nix {};

      devShells.default = pkgs.mkShell {
        buildInputs = [ (pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        }) ];
      };
    }
  );
}
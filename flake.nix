{
  description = "Typst Detexify";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, utils, fenix, }:
    utils.lib.eachDefaultSystem (system:
    let
        fenixStable = with fenix.packages.${system}; combine [
            (stable.withComponents [ "cargo" "clippy" "rust-src" "rustc" "rustfmt" "llvm-tools-preview" ])
            targets.wasm32-unknown-unknown.stable.rust-std
          ];
        overlaid = final: prev:
          {
          };
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            overlaid
          ];
        };
        in {
          devShell = pkgs.mkShell.override { } {
            shellHook = ''
              export CARGO_TARGET_DIR="$(git rev-parse --show-toplevel)/target_ditrs/nix_rustc";
            '';
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
            buildInputs =
              with pkgs; [
                typst
                pandoc
                wasm-bindgen-cli
                fenixStable
                fenix.packages.${system}.rust-analyzer
                just
                cargo-expand
                wasm-pack
                corepack
                python3
              ] ++
              pkgs.lib.optionals stdenv.isDarwin [
                darwin.apple_sdk.frameworks.Security
                pkgs.libiconv
                darwin.apple_sdk.frameworks.SystemConfiguration
              ];
          };
    });
}

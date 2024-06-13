{inputs, ...}: {
  perSystem = {
    l,
    pkgs,
    config,
    inputs',
    ...
  }: let
    fenix = inputs'.fenix.packages;

    toolchain = with fenix;
      combine [
        (stable.withComponents ["cargo" "clippy" "rust-src" "rustc" "rustfmt" "llvm-tools-preview"])
        targets.wasm32-unknown-unknown.stable.rust-std
      ];

    craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;

    commonArgs = {
      src = ./.;

      version = "0.1.0";
      strictDeps = true;

      buildInputs = l.optionals pkgs.stdenv.isDarwin [
        pkgs.libiconv
        pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
      ];

      doCheck = false;
    };
  in {
    treefmt.config.programs.rustfmt.enable = true;

    packages.app = craneLib.buildPackage (commonArgs
      // {
        pname = "app";
        cargoExtraArgs = "--package app --target wasm32-unknown-unknown";
        CARGO_BUILD_RUSTFLAGS = "-Ctarget-feature=+atomics,+bulk-memory,+mutable-globals -C embed-bitcode=yes -C codegen-units=1 -C opt-level=3 --cfg web_sys_unstable_apis";
      });

    packages.scraper = craneLib.buildPackage (commonArgs
      // {
        pname = "scraper";
      });

    devShells.rust = pkgs.mkShell {
      inputsFrom = [config.packages.app config.packages.scraper];
      packages = l.attrValues {
        inherit (pkgs) dioxus-cli;
        inherit (fenix) rust-analyzer;
        inherit (config.treefmt.build.programs) rustfmt;
      };
    };
  };
}

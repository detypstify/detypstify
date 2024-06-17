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
        (stable.withComponents ["cargo" "clippy" "rust-src" "rustc" "llvm-tools-preview"])
        targets.wasm32-unknown-unknown.stable.rust-std
      ];

    craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;

    # HACK: Workspace package versions don't seem to be detected by crane
    readPackageVersion = file:
      (l.fromTOML (l.readFile file)).package.version;

    common.buildInputs = l.optionals pkgs.stdenv.isDarwin (l.attrValues {
      inherit (pkgs) libiconv;
      inherit (pkgs.darwin.apple_sdk.frameworks) SystemConfiguration;
    });
  in {
    treefmt.config.programs.rustfmt.enable = true;

    # NOTE: App is not part of the workspace
    packages.app = let
      app-config = l.fromTOML (l.readFile ./apps/web-dioxus/.cargo/config.toml);

      dioxus-cli = pkgs.dioxus-cli.overrideAttrs (ol: rec {
        src = pkgs.fetchFromGitHub {
          owner = "DioxusLabs";
          repo = "dioxus";
          rev = "487570d89751b34bbfd5e9b5ff1e0fd3780bf332";
          sha256 = "xWAnDZcNhVLa8PckWGxk8gU6qihb87e/aTfoKls5KL8=";
        };

        buildAndTestSubdir = "./packages/cli";

        cargoDeps = ol.cargoDeps.overrideAttrs (l.const {
          name = "dioxus-cli-vendor.tar.gz";
          inherit src;
          outputHash = "sha256-ox1QtI9qUwKfN8SjIRQSD3rS/ZyKuQ07vC71YABEHoQ=";
        });
      });
    in
      craneLib.buildPackage {
        pname = "app";
        src = ./apps/web-dioxus;

        buildInputs =
          l.attrValues {
            inherit (pkgs) pkg-config openssl;
          }
          ++ common.buildInputs;

        nativeBuildInputs = l.attrValues {
          inherit dioxus-cli;
          inherit (pkgs) tailwindcss;
          inherit (config.treefmt.build.programs) rustfmt; # Yes, for some reason burn depends on rustfmt
        };

        strictDeps = true;
        doCheck = false;

        CARGO_BUILD_TARGET = app-config.build.target;
        CARGO_BUILD_RUSTFLAGS = l.concatStringsSep " " app-config.build.rustflags;
      };

    packages.dataset = craneLib.buildPackage {
      pname = "dataset";
      version = readPackageVersion ./crates/dataset/Cargo.toml;
      src = l.fileset.toSource {
        root = ./.;
        fileset = l.fileset.unions [
          ./Cargo.toml
          ./Cargo.lock

          ./crates/dataset
        ];
      };

      inherit (common) buildInputs;

      strictDeps = true;
      cargoExtraArgs = "-p dataset";
    };

    devShells.rust = pkgs.mkShell {
      inputsFrom = [config.packages.app config.packages.dataset];
      packages = l.attrValues {
        inherit (pkgs) tailwindcss-language-server;
        inherit (fenix) rust-analyzer;
        inherit (config.treefmt.build.programs) rustfmt;
      };
    };
  };
}

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
      app-config = l.fromTOML (l.readFile ./app/.cargo/config.toml);
    in
      craneLib.buildPackage {
        pname = "app";
        src = ./app;

        inherit (common) buildInputs;
        nativeBuildInputs = l.attrValues {
          inherit (pkgs) dioxus-cli tailwindcss;
          inherit (config.treefmt.build.programs) rustfmt; # Yes, for some reason burn depends on rustfmt
        };

        strictDeps = true;

        CARGO_BUILD_TARGET = app-config.build.target;
        CARGO_BUILD_RUSTFLAGS = l.concatStringsSep " " app-config.build.rustflags;
      };

    packages.scraper = craneLib.buildPackage {
      pname = "scraper";
      version = readPackageVersion ./scraper/Cargo.toml;
      src = l.fileset.toSource {
        root = ./.;
        fileset = l.fileset.unions [
          ./Cargo.toml
          ./Cargo.lock

          ./scraper
        ];
      };

      inherit (common) buildInputs;

      strictDeps = true;
      cargoExtraArgs = "-p scraper";
    };

    devShells.rust = pkgs.mkShell {
      inputsFrom = [config.packages.app config.packages.scraper];
      packages = l.attrValues {
        inherit (pkgs) tailwindcss-language-server;
        inherit (fenix) rust-analyzer;
        inherit (config.treefmt.build.programs) rustfmt;
      };
    };
  };
}

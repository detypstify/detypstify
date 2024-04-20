{
  description = "Typst Detexify";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixified-ai = {
      url = "github:DieracDelta/flake/jr";
    };
    nixified-ai-new = {
      url = "github:nixified-ai/flake";
    };
  };

  outputs = inputs@{ self, nixpkgs, utils, fenix, nixified-ai, nixified-ai-new}:
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
          config = {
            cudaSupport = true;
            allowUnfree = true;
          #   rocmSupport = true;
          };
          overlays = [
            nixified-ai-new.overlays.python-torchCuda
          ];
        };
        pkgs_mvp = import nixpkgs {
          inherit system;
          config = {
            allowUnfree = true;
          };
        };
        in {
          defaultPackage = self.devShell.${system};
          devShells = {
            mvp = pkgs_mvp.mkShell {
              buildInputs = with pkgs; [
                ninja
                pkg-config
                (python311.withPackages (ps: with ps; [ (torch.override { cudaSupport =  true; } /* .override { cudaSupport = true; } */ ) torchvision numpy pip python scikit-learn datasets transformers jiwer jupyter ipywidgets ] ))
              ];


            };
          };
          devShell = pkgs.mkShell.override { } {
            shellHook = ''
              export CARGO_TARGET_DIR="$(git rev-parse --show-toplevel)/target_ditrs/nix_rustc";
            '';
            LD_LIBRARY_PATH = "${pkgs.xorg.libX11}/lib:${pkgs.xorg.libXcursor}/lib:${pkgs.xorg.libXrandr}/lib:${pkgs.xorg.libXi}/lib:${pkgs.libxkbcommon}/lib:${pkgs.vulkan-loader}/lib:${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.libGL}/lib:${pkgs.glib.out}/lib";
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

                ## python deps for VIT
                # python312
                ninja
                vulkan-loader
                xorg.libX11.dev
                xorg.libX11
                libxkbcommon
                fontconfig
                typst
                typst-lsp
                openblas
                cargo-expand
                boost
                pkg-config
                openssl.dev
                openssl
                (python311.withPackages (ps: with ps; [ (torch /* .override { cudaSupport = true; } */ ) torchvision numpy pip python scikit-learn datasets transformers jiwer jupyter ipywidgets ] ))
                # A Python interpreter including the 'venv' module is required to bootstrap
                # the environment.
                # python310Packages.python
                # python310Packages.pip
                #
                # # This executes some shell code to initialize a venv in $venvDir before
                # # dropping into the shell
                # python312Packages.venvShellHook
                # python310Packages.numpy
                # python312Packages.requests
                # python312Packages.pre-commit-hooks
                #
                # # Working with data
                # python312Packages.pandas
                # python312Packages.numpy
                # python312Packages.matplotlib
                # python312Packages.seaborn
                # python312Packages.kaggle
                # python312Packages.phik
                # python312Packages.tqdm
                # python312Packages.wandb

                # Machine Learning
                # python312Packages.scikit-learn
                # python312Packages.xgboost
                # python312Packages.lightgbm
                # python312Packages.catboost
                # python310Packages.torchWithRocm
                # python310Packages.torchvision
                # python312Packages.torchaudio

                # Web Frameworks
                # streamlit
                # python312Packages.jupyter
                # python312Packages.ipykernel


              ] ++
              pkgs.lib.optionals stdenv.isDarwin [
                darwin.apple_sdk.frameworks.Security
                pkgs.libiconv
                darwin.apple_sdk.frameworks.SystemConfiguration
              ];
          };
    });
}

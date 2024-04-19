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
        pkgs_old = import nixified-ai.inputs.nixpkgs {
          inherit system;
          # overlays = [() ];
        };
        pkgs_new = import nixpkgs {
          inherit system;
          config = {
            # cudaSupport = true;
          #   rocmSupport = true;
          };
          overlays = [
          # (
          #   self: super: {
          #     python311 = super.python311.override {
          #       packageOverrides = python-self: python-super: {
          #         torch-bin = python-super.torch-bin.overrideAttrs (oldAttrs: {
          #             src = pkgs_new.fetchurl { name = "torch-2.0.1+rocm5.7-cp311-cp311-linux_x86_64.whl";
          #             url = "https://repo.radeon.com/rocm/manylinux/rocm-rel-5.7/torch-2.0.1%2Brocm5.7-cp311-cp311-linux_x86_64.whl";
          #             hash = "sha256-h4MIjra94HdalAaMo4/mVsK1a6R/cluEsFa0ISAYPY4=";
          #
          #             };
          #             postFixup =
          #             ''
          #               addAutoPatchelfSearchPath "$out/${python-super.python.sitePackages}/torch/lib"
          #               addAutoPatchelfSearchPath "${pkgs_new.rocmPackages_5.rocblas}/lib"
          #               pushd $out/${python-super.python.sitePackages}/torch/lib || exit 1
          #                 for LIBNVRTC in ./libnvrtc*
          #                 do
          #                   case "$LIBNVRTC" in
          #                     ./libnvrtc-builtins*) true;;
          #                     ./libnvrtc*) patchelf "$LIBNVRTC" --add-needed libnvrtc-builtins* ;;
          #                   esac
          #                 done
          #               popd || exit 1
          #             '';
          #         });
          #       };
          #     };
          #   }
          # )
          ];
        };
        in {
          devShells = {
            oldaf = pkgs_old.mkShell {
              "HSA_OVERRIDE_GFX_VERSION" = "10.3.0";
              LD_LIBRARY_PATH = "${pkgs_old.xorg.libX11}/lib:${pkgs_old.xorg.libXcursor}/lib:${pkgs_old.xorg.libXrandr}/lib:${pkgs_old.xorg.libXi}/lib:${pkgs_old.libxkbcommon}/lib:${pkgs_old.vulkan-loader}/lib:${pkgs_old.stdenv.cc.cc.lib}/lib:${pkgs_old.libGL}/lib:${pkgs_old.glib.out}/lib:${pkgs_old.rocblas}/lib:${pkgs_old.hip}/lib";
              buildInputs = with pkgs_old; [
                nixified-ai.packages.${system}.numpy
                nixified-ai.packages.${system}.pip
                nixified-ai.packages.${system}.python
                nixified-ai.packages.${system}.torch
                # nixified-ai.packages.${system}.jiwer
                nixified-ai.packages.${system}.transformers
                nixified-ai.packages.${system}.datasets
                nixified-ai.packages.${system}.scikit-learn
                hip

                libdrm
                ninja
                pkg-config
                openssl.dev
                openssl
              ];
            };
            newaf = pkgs_new.mkShell {
              "HSA_OVERRIDE_GFX_VERSION" = "10.3.0";

              LD_LIBRARY_PATH = "${pkgs_new.xorg.libX11}/lib:${pkgs_new.xorg.libXcursor}/lib:${pkgs_new.xorg.libXrandr}/lib:${pkgs_new.xorg.libXi}/lib:${pkgs_new.libxkbcommon}/lib:${pkgs_new.vulkan-loader}/lib:${pkgs_new.stdenv.cc.cc.lib}/lib:${pkgs_new.libGL}/lib:${pkgs_new.glib.out}/lib:${pkgs_new.rocmPackages_5.rocblas}/lib:${pkgs_new.rocmPackages_5.hip-common}/lib:${pkgs_new.rocmPackages_5.migraphx}/lib:${pkgs_new.rocmPackages_5.miopen}/lib:${pkgs_new.rocmPackages_5.hipfft}/lib:${pkgs_new.rocmPackages_5.miopen-hip}/lib:${pkgs_new.rocmPackages_5.roctracer}/lib:${pkgs_new.rocmPackages_5.clr}/lib:${pkgs_new.rocmPackages_5.clr}/hip/lib";
              buildInputs = with pkgs_new; [
                rocmPackages.rocminfo
                rocmPackages.rocm-core
                # python310Packages.numpy
                python311Packages.pip
                python311Packages.python
                python311Packages.torch-bin

                # python311Packages.torch-bin.overrideAttrs (prev: {

                # })

                # python310Packages.torchWithRocm
                # python310Packages.torchvision
                # python310Packages.pandas
                # python310Packages.jiwer
                # python310Packages.transformers
                # python310Packages.datasets
                # python310Packages.scikit-learn

                libdrm
                ninja
                pkg-config
                openssl.dev
                openssl
              ];

            };
          };
          defaultPackage = self.devShell.${system};
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
                (python311.withPackages (ps: with ps; [ (torch /* .override { cudaSupport = true; } */ ) torchvision numpy pip python scikit-learn datasets transformers]))
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

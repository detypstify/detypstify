{
  description = "Typst Detexify";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # nixified-ai = {
    #   url = "github:DieracDelta/flake/jr";
    # };
    # nixified-ai-new = {
    #   url = "github:nixified-ai/flake";
    # };
  };

  outputs = inputs@{ self, nixpkgs, utils, fenix}:
    utils.lib.eachDefaultSystem (system:
    let
        fenixStable = with fenix.packages.${system}; combine [
            (stable.withComponents [ "cargo" "clippy" "rust-src" "rustc" "rustfmt" "llvm-tools-preview" ])
            targets.wasm32-unknown-unknown.stable.rust-std
          ];
        # overlaid = final: prev:
        #   {
        #   };
        # pkgs = import nixpkgs {
        #   inherit system;
        #   config = {
        #     cudaSupport = true;
        #     allowUnfree = true;
        #   #   rocmSupport = true;
        #   };
        #   overlays = [
        #     nixified-ai-new.overlays.python-torchCuda
        #   ];
        # };
        pkgs = import nixpkgs {
          inherit system;
          config = {
            allowUnfree = true;
          };
        };
        in {
          defaultPackage = self.devShell.${system};
          devShell = pkgs.mkShell.override { } {
            shellHook = ''
              export CARGO_TARGET_DIR="$(git rev-parse --show-toplevel)/target_ditrs/nix_rustc";
            '';
            # LD_LIBRARY_PATH = "${pkgs.xorg.libX11}/lib:${pkgs.xorg.libXcursor}/lib:${pkgs.xorg.libXrandr}/lib:${pkgs.xorg.libXi}/lib:${pkgs.libxkbcommon}/lib:${pkgs.vulkan-loader}/lib:${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.libGL}/lib"; /* :${pkgs.glib.out}/lib"; */
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
            buildInputs =
              with pkgs; [
                ninja
                pkg-config
                # (python311.withPackages (ps: with ps; [ (torch.override { cudaSupport =  true; } /* .override { cudaSupport = true; } */ ) /* torchvision */ numpy pip python scikit-learn datasets transformers jiwer jupyter ipywidgets onnxruntime onnx ] ))
                typst
                pandoc
                wasm-bindgen-cli
                fenixStable
                # fenix.packages.${system}.rust-analyzer
                just
                cargo-expand
                wasm-pack
                corepack

                ninja
                # vulkan-loader
                # xorg.libX11.dev
                # xorg.libX11
                # libxkbcommon
                nodejs
                fontconfig
                # typst
                # typst-lsp
                # openblas
                # cargo-expand
                # boost
                # pkg-config
                openssl.dev
                openssl

              ];
          };
    });
}

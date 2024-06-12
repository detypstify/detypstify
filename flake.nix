{
  description = "detypstify: Using OCR to convert images of formulas into Typst code.";

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-darwin" "x86_64-darwin"];
      imports = [
        {perSystem = {lib, ...}: {_module.args.l = lib // builtins;};}
        inputs.treefmt-nix.flakeModule
      ];
      perSystem = {
        l,
        pkgs,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          packages = l.attrValues {
            inherit (pkgs) just;
          };
        };

        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            ruff.enable = true;
            alejandra.enable = true;
            rustfmt.enable = true;
          };
        };
      };
    };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}

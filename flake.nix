{
  description = "detypstify: Using OCR to convert images of formulas into Typst code.";

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-darwin" "x86_64-darwin"];
      imports = [
        {perSystem = {lib, ...}: {_module.args.l = lib // builtins;};}

        inputs.treefmt-nix.flakeModule
        ./python.flakeModule.nix
        ./paper/flakeModule.nix
      ];
      perSystem = {
        l,
        pkgs,
        config,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          inputsFrom = [
            config.devShells.python
            config.devShells.paper
          ];

          packages = l.attrValues {
            inherit (pkgs) just;
          };
        };

        treefmt.config = {
          projectRootFile = "flake.nix";
          programs.alejandra.enable = true;
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

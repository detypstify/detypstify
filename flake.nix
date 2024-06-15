{
  description = "detypstify: Using OCR to convert images of formulas into Typst code.";

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-darwin" "x86_64-darwin"];
      imports = [
        {
          perSystem = {lib, system, ...}: {
            _module.args.l = lib // builtins;
            _module.args.pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [(final: prev: {
                            dioxus-cli = prev.rustPlatform.buildRustPackage {
                              name = "dioxus-cli";
                              src = prev.fetchFromGitHub {
                                owner = "DioxusLabs";
                                repo = "dioxus";
                                rev = "main";
                                hash = "sha256-xWAnDZcNhVLa8PckWGxk8gU6qihb87e/aTfoKls5KL8=";
                              };

                              cargoHash = "sha256-ox1QtI9qUwKfN8SjIRQSD3rS/ZyKuQ07vC71YABEHoQ=";

                              nativeBuildInputs = with prev.pkgs; [ pkg-config cacert ];
                              buildInputs = [ prev.pkgs.openssl ] ++ lib.optionals prev.stdenv.isDarwin [
                                prev.darwin.apple_sdk.frameworks.CoreServices
                                prev.darwin.apple_sdk.frameworks.SystemConfiguration
                              ];

                              buildAndTestSubdir = "./packages/cli";

                              OPENSSL_NO_VENDOR = 1;

                              nativeCheckInputs = [ prev.pkgs.rustfmt ];

                              doCheck = false;

                              meta = with lib; {
                                homepage = "https://dioxuslabs.com";
                                description = "CLI tool for developing, testing, and publishing Dioxus apps";
                                license = with licenses; [ mit asl20 ];
                                maintainers = with maintainers; [ xanderio cathalmullan ];
                                mainProgram = "dx";
                              };
                            };
              })];
            };
          };
        }

                # prev.dioxus-cli.overrideAttrs (old: {
                #   src = prev.fetchFromGitHub {
                #     owner = "DioxusLabs";
                #     repo = "dioxus";
                #     rev = "v0.5.1";
                #     hash = "sha256-S2yX/cZk+VaiTr6TLCU7/LmusKsD1T/CXqCJqohk/Eo=";
                #   };
                #
                #   cargoHash = "sha256-S2yX/cZk+VaiTr6TLCU7/LmusKsD1T/CXqCJqohk/Eo=";
                #   # cargoSha256 = "";
                #   version = "0.5.1";
                # });


        inputs.treefmt-nix.flakeModule
        ./rust.flakeModule.nix
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
            config.devShells.rust
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

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    typst-packages = {
      url = "github:typst/packages";
      flake = false;
    };
  };
}

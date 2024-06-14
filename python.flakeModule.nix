{
  perSystem = {
    l,
    pkgs,
    config,
    ...
  }: {
    treefmt.config.programs.ruff = {
      check = true;
      format = true;
    };

    devShells.python = pkgs.mkShell {
      packages = l.attrValues {
        inherit (pkgs) poetry python313;
        inherit (config.treefmt.build.programs) ruff;
      };
    };
  };
}

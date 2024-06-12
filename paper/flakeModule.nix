{
  perSystem = {
    l,
    pkgs,
    ...
  }: {
    devShells.paper = pkgs.mkShell {
      packages = l.attrValues {
        inherit (pkgs) typst typst-preview typstyle tinymist;
      };
    };
  };
}

# vim: ts=2 sw=0 et ft=nix
{
  inputs.fia.url = github:poollovernathan/fia;
  outputs = { self, fia }: {
    packages = fia.lib.perSystem (pkgs: {
      default = fia.lib.crossCompile' {
        src = ./.;
        pkgs = pkgs;
        target = null;
      };
    });
  };
}


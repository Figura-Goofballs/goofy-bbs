# vim: ts=2 sw=0 et ft=nix
{
  inputs.fia.url = github:poollovernathan/fia;
  outputs = { self, fia }: {
    packages = fia.lib.perSystem (pkgs: {
      default = fia.lib.crossCompile' {
        src = ./.;
        pkgs = pkgs;
        target = null;
        attrs.buildInputs = [pkgs.acl.out pkgs.sqlite.out];
        attrs.cargoBuildFlags = ["-vvv"];
        attrs.LIBRARY_PATH = "${pkgs.acl.out}";
        attrs.CARGO_BUILD_RUSTFLAGS = [
          # libacl really hates me
          "-C" "link-args=-v -L${pkgs.acl.out}/lib"
          "-v"
        ];
      };
    });
  };
}


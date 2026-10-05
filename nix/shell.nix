{
  mkShell,
  rustc,
  cargo,
  rust-analyzer,
  clippy,
  rustfmt,
  taplo,
  djlint,
}:
mkShell {
  name = "wat-dev";
  packages = [
    rustc
    cargo

    rust-analyzer
    clippy
    rustfmt
    taplo
    djlint
  ];
}

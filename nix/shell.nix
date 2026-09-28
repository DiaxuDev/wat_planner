{
  mkShell,
  rustc,
  cargo,
  rust-analyzer,
  clippy,
  rustfmt,
  taplo,
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
  ];
}

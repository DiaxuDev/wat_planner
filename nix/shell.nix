{
  mkShell,
  rust,
  taplo,
  djlint,
  cargo-ndk,
}:
mkShell {
  name = "wat-dev";
  packages = [
    rust
    taplo
    djlint
    cargo-ndk
  ];
}

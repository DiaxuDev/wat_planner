{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:Oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (
        system: pkgs: {
          wat_planner = pkgs.callPackage ./nix/package.nix { };
          default = self.packages.${system}.wat_planner;
        }
      );

      devShells = forAllSystems (
        _: pkgs:
        let
          rust = (rust-overlay.lib.mkRustBin { } pkgs).stable.latest.default.override {
            targets = [
              "x86_64-linux-android"
              "aarch64-linux-android"
            ];
            extensions = [
              "rust-analyzer"
              "rust-src"
            ];
          };
        in
        {
          default = pkgs.callPackage ./nix/shell.nix { inherit rust; };
        }
      );
    };
}

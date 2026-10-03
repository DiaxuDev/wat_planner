{
  lib,
  rustPlatform,
}:
let
  cargoToml = (lib.importTOML ../Cargo.toml).package;
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = cargoToml.name;
  version = cargoToml.version;

  __structuredAttrs = true;

  src =
    let
      root = ../.;
    in
    lib.fileset.toSource {
      inherit root;
      fileset = lib.fileset.unions [
        (root + /src)
        (root + /include/template.html)
        (root + /Cargo.toml)
        (root + /Cargo.lock)
      ];
    };

  cargoLock.lockFile = "${finalAttrs.src}/Cargo.lock";
  enableParallelBuilding = true;

  meta = {
    description = "WAT schedule parser and presenter";
    homepage = "github.com/DiaxuDev/wat_planner";
    license = lib.licenses.mpl20;
    mainProgram = "wat_planner";
  };
})

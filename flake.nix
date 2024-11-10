{
  description = "NeoHTop flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    nixpkgs-getchoo-tauri.url = "github:getchoo-contrib/nixpkgs?ref=pkgs/cargo-tauri/2.0.1";
  };

  outputs = inputs@{ nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      systems = ["aarch64-darwin"];
      forAllSystems = lib.genAttrs systems;
      spkgs = system : nixpkgs.legacyPackages.${system}.pkgs;
    in
    {
      packages = forAllSystems (system: with spkgs system; rec {
        getchoo = inputs.nixpkgs-getchoo-tauri.legacyPackages.${system};
        tauri = getchoo.pkgs.cargo-tauri;
        neohtop = rustPlatform.buildRustPackage rec {
          name = "neoHtop";
          src = ./.;
          nativeBuildInputs = [ tauri.hook npmHooks.npmConfigHook nodejs ];
          cargoRoot = "src-tauri";
          cargoLock = {
            lockFile = src-tauri/Cargo.lock;
          };
          npmDeps = importNpmLock {
            npmRoot = ./.;
          };
        };
        default = neohtop;
      });
      devShells = forAllSystems (system: with spkgs system; {
        default = mkShell {
          buildInputs = [ cargo rustc nodejs ];
        };
      });
    };
}

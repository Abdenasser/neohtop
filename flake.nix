{
  description = "NeoHTop flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    # as upstream nixpkgs doesn't have tauri 2, we use a fork
    nixpkgs2.url = "github:hannesGitH/nixpkgs?ref=tauri2";
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
        remote2 = inputs.nixpkgs2.legacyPackages.${system};
        tauri = remote2.pkgs.cargo-tauri;
        neohtop = rustPlatform.buildRustPackage rec {
          name = "neoHtop";
          src = ./.;
          nativeBuildInputs = [ tauri.hook npmConfigHook nodejs ];
          cargoRoot = "src-tauri";
          cargoLock = {
            lockFile = src-tauri/Cargo.lock;
          };
          npmDeps = importNpmLock {
            npmRoot = ./.;
          };
          patches = [
            ./nix/patches/dontbundle.patch
          ];
          tauriFlags = "-vvv";
          npmConfigHook = importNpmLock.npmConfigHook;
          # npmConfigHook = npmHooks.npmConfigHook;
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

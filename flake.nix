{
  description = "NeoHTop flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      systems = ["aarch64-darwin"];
      forAllSystems = lib.genAttrs systems;
      spkgs = system : nixpkgs.legacyPackages.${system}.pkgs;
    in
    {
      packages = forAllSystems (system: with spkgs system; rec {
        # tauri = callPackage ./nix/pkgs/tauri.nix { };
        # taurihook = cargo-tauri.hook.override { cargo-tauri = tauri; };
        # neohtop = buildNpmPackage {
        #   name = "neohtop";
        #   src = ./.;
        #   nativeBuildInputs = [ taurihook ];
        #   cargoRoot = "src-tauri";
        #   npmDeps = importNpmLock {
        #     npmRoot = ./.;
        #   };
        #   npmConfigHook = importNpmLock.npmConfigHook;
        # };
        # default = neohtop;
      });
      devShells = forAllSystems (system: with spkgs system; {
        default = mkShell {
          buildInputs = [ cargo rustc nodejs ];
        };
      });
    };
}

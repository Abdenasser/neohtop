{ fetchFromGitHub
, cargo-tauri
, rustPlatform
}:
let
  version = "2.1.0";
  src = fetchFromGitHub {
    owner = "tauri-apps";
    repo = "tauri";
    rev = "tauri-v${version}";
    hash = "sha256-7usmUaAK9HIjrBvq+wquAlI5KuAQA/VFhDp6trxvDAQ=";
  };
in cargo-tauri.overrideAttrs (drv: rec {
  inherit src version;
  # cargoDeps = drv.cargoDeps.overrideAttrs (lib.const {
  #   inherit src;
  #   name = "tauri-${version}-vendor.tar.gz";
  #   sourceRoot = "${src.name}/tooling/cli";
  # });
  # # buildInputs = [ gcc clang pkg-config ];
  patchPhase = ''
    cp -r ../../Cargo.lock .
  '';
  cargoDeps = rustPlatform.importCargoLock {
    lockFile = "${src}/Cargo.lock";
    outputHashes = {
      "schemars_derive-0.8.21" = "sha256-AmxBKZXm2Eb+w8/hLQWTol5f22uP8UqaIh+LVLbS20g=";
    };
  };
  sourceRoot = "${src.name}/crates/tauri-cli";
})
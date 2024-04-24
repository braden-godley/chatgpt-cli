{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "chatgpt-cli";
  version = "0.1";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  buildInputs = [
    pkgs.openssl
  ];

  nativeBuildInputs = [
    pkgs.openssl
    pkgs.pkg-config
  ];
}

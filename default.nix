{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "foo-bar";
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

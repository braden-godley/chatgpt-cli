{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.openssl
    pkgs.pkg-config
    pkgs.rustup
    pkgs.cargo
  ];

  # shellHook = ''
  #   export OPENSSL_DIR=${pkgs.openssl.dev}
  #   export PKG_CONFIG_PATH=${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH
  # '';
}

{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    openssl
    rustup
  ];
  shellHook = ''
    rustup component add rust-analyzer
  '';
}

{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = with pkgs.buildPackages; [
    cargo
    SDL2
    rustup
    rustc
  ];

  # shellHook = ''
  #   # ...
  # '';
}

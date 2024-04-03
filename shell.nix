{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = with pkgs.buildPackages; [
    cargo
    SDL2
    SDL2_gfx
    rustup
    rustc
    dhex
  ];

  # shellHook = ''
  #   # ...
  # '';
}

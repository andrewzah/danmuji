{ pkgs ? import <nixpkgs> {} }: with pkgs;

let
  #danmuji = rustPlatform.buildRustPackage rec {
    #name = "danmuji-${version}";
    #version = "0.1.0";
    #src = ${
  #};
  #danmuji = stdenv.mkDerivation { # ??
    #name = "danmuji";
    #src = 
  #};
  danmuji = /nix/store/b23ll1r78y6s89k0b4rc625f68x0k9mg-rust_danmuji-0.1.0;
in
  dockerTools.buildLayeredImage {
    name = "andrewzah/alpine-rust-nix";
    tag = "latest";
    created = "now";

    contents = [ postgresql.lib danmuji ];
    #contents = [ (callPackage ./Cargo.nix {}).__all ];
  }

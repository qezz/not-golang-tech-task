let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  r = (nixpkgs.rustChannelOf { date = "2020-12-18"; channel = "nightly"; }).rust;
  rust-nightly = (r.override {
    extensions = [
      "rust-src"
      "rls-preview"
      "rust-analysis"
      "rustfmt-preview"
    ];
  });

in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "moz_overlay_shell";
    buildInputs = [
      rust-nightly
      pkg-config
      postgresql
      openssl

      diesel-cli
    ];
  }

{ pkgs ? import <nixpkgs> { } }:

let
  rust_overlay = import (builtins.fetchTarball {
    url = "https://github.com/oxalica/rust-overlay/archive/a16b9a7cac7f4d39a84234d62e91890370c57d76.tar.gz";
    sha256 = "sha256:05xyk469bj6zkvkk4gmc58rkiyavamn4xhfglwkdqlanqiyfwdfz";
  });
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rust = pkgs.rust-bin.stable."1.81.0".default.override {
    extensions = [ "rust-src" ];
  };
in
pkgs.mkShell {
  buildInputs = [
    rust
  ] ++ (with pkgs; [
    pkg-config
    rust-analyzer
    sccache
  ]);

  RUST_BACKTRACE = 1;
  RUSTC_WRAPPER = "sccache";
  SCCACHE_SERVER_PORT = "54226";
}

{
  description = "Tea Master - simple counter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl
            fish
          ];
          shellHook = ''
            exec ${pkgs.fish}/bin/fish -C "
              alias tea_master='rustc ./tea_master.rs && ./tea_master'
              set fish_greeting \"
  Rust dev environment
  tea_master - build and run ./tea_master
              \"
            "
          '';

          RUST_BACKTRACE = 1;
        };
      });
}

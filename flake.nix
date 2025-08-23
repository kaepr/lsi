{
  description = "Simple Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain from nixpkgs
            rustc
            cargo
            rust-analyzer
            rustfmt
            clippy

            # Development tools
            pkg-config
            openssl

            # Optional utilities
            cargo-watch
            cargo-edit
          ];

          shellHook = ''
            echo "🦀 Rust development environment ready!"
            echo "Rust: $(rustc --version)"
            echo "Cargo: $(cargo --version)"
          '';
        };
      }
    );
}

{
    description = "A commandline menu generator";
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs = {
                nixpkgs.follows = "nixpkgs";
                flake-utils.follows = "flake-utils";
            };
        };
    };
    outputs = { self, nixpkgs, flake-utils, rust-overlay }:
        flake-utils.lib.eachDefaultSystem
            (system:
                let
                    overlays = [ (import rust-overlay) ];
                    pkgs = import nixpkgs {
                        inherit system overlays;
                    };
                    rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
                    nativeBuildInputs = with pkgs; [ rustToolchain ];
                    buildInputs = with pkgs; [ ];
                in
                with pkgs;
                {
                    devShells.default = mkShell {
                        inherit buildInputs nativeBuildInputs;
                    };
                }
            );
}

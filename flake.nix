{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        naersk-lib = naersk.lib.${system}.override { };
      in
      {
        packages = rec {
          default = nothingverse;
          nothingverse = naersk-lib.buildPackage {
            src = ./.;
          };
        };

        devShells = {
          default = pkgs.mkShell {
            packages = with pkgs; [
              pkg-config
              openssl
              rust-bin.stable.latest.default
            ];
          };
        };
      }
    );
}

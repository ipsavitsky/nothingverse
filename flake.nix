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
        naersk-lib = naersk.lib.${system}.override {
          cargo = pkgs.rust-bin.stable.latest.default;
          rustc = pkgs.rust-bin.stable.latest.default;
        };
        chunkyFont = pkgs.fetchurl {
          url = "http://www.figlet.org/fonts/chunky.flf";
          hash = "sha256-A0BwES9Pz9YT8GDxrg5ECHDe/fNjrrMKtG9mnzPXsXM=";
        };
      in
      {
        packages = rec {
          default = nothingverse;
          nothingverse = naersk-lib.buildPackage {
            src = ./.;
            nativeBuildInputs = with pkgs; [
              toilet
            ];
            overrideMain = old: {
              preBuild = ''
                # for some reason I have to symlink or toilet can't open the font
                ln -s ${chunkyFont} chunky.flf
                toilet -f ./chunky.flf nothingverse --html > templates/logo.html
              '';
            };
          };
        };

        devShells = {
          default = pkgs.mkShell {
            packages = with pkgs; [
              pkg-config
              openssl
              toilet
              rust-analyzer
              cargo-watch
              just
              wget
              rust-bin.stable.latest.default
            ];
          };
        };
      }
    );
}

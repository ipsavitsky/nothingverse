{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
      rust-overlay,
      treefmt-nix,
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
        treefmtModule = treefmt-nix.lib.evalModule pkgs ./nix/treefmt.nix;
      in
      {
        formatter = treefmtModule.config.build.wrapper;

        checks = {
          formatting = treefmtModule.config.build.check self;
        };

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
              SQLX_OFFLINE = true;
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
              watchexec
              just
              wget
              sqlx-cli
              sqlite
              rust-bin.stable.latest.default
            ];
          };
        };
      }
    );
}

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
        naersk-lib = pkgs.callPackage naersk {
          cargo = pkgs.rust-bin.stable.latest.default;
          rustc = pkgs.rust-bin.stable.latest.default;
        };
        chunkyFont = pkgs.fetchurl {
          url = "http://www.figlet.org/fonts/chunky.flf";
          hash = "sha256-A0BwES9Pz9YT8GDxrg5ECHDe/fNjrrMKtG9mnzPXsXM=";
        };
        npmHash = import ./nix/npmHash.nix;
        treefmtModule = treefmt-nix.lib.evalModule pkgs ./nix/treefmt.nix;
      in rec
      {
        formatter = treefmtModule.config.build.wrapper;

        checks = {
          formatting = treefmtModule.config.build.check self;
          x86_64-linux = packages.nothingverse;
        };

        packages = rec {
          default = nothingverse;
          nothingverse = naersk-lib.buildPackage {
            src = ./.;
            nativeBuildInputs = with pkgs; [
              toilet
              htmlq
              tailwindcss_4
            ];
            overrideMain = old: {
              nativeBuildInputs = old.nativeBuildInputs ++ [
                pkgs.nodejs
                pkgs.npmHooks.npmConfigHook
              ];
              npmDeps = pkgs.fetchNpmDeps {
                src = ./.;
                hash = npmHash;
              };
              preBuild = ''
                # for some reason I have to symlink or toilet can't open the font
                tailwindcss -i ./templates/styles-in.css -o ./templates/styles.css
                ln -s ${chunkyFont} chunky.flf
                toilet -f ./chunky.flf nothingverse --html | htmlq 'body' > templates/logo.html
              '';
              SQLX_OFFLINE = true;
            };
          };
          modelfile = pkgs.writeTextFile {
            name = "modelfile";
            text = builtins.readFile ./ollama/nothing.modelfile;
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
              tailwindcss_4
              htmlq
              zizmor
              rust-bin.stable.latest.default
              nodejs
              nil
            ];
          };
        };
      }
    )
    // {
      nixosModules = {
        nothingverse = import ./nix/service.nix { nothingverse = self.packages; };
      };
    };
}

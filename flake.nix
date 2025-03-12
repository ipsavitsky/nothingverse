{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
    statix.url = "github:oppiliappan/statix";
    nom.url = "github:maralorn/nix-output-monitor";
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
      statix,
      nom,
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
        htmx_org = pkgs.fetchurl {
          url = "https://unpkg.com/htmx.org@2.0.4/dist/htmx.min.js";
          hash = "sha256-4gndpcgjVHnzFm3vx3UOHbzVpcGAi3eS/C5nM3aPtEc=";
        };
        htmx_ext_sse = pkgs.fetchurl {
          url = "https://unpkg.com/htmx-ext-sse@2.2.3/dist/sse.min.js";
          hash = "sha256-IEoX7Cv0kLffWS9V6+VH5EsJn7U8iLRC0LztDhAyfhI=";
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
              tailwindcss_4
            ];
            overrideMain = old: {
              preBuild = ''
                # for some reason I have to symlink or toilet can't open the font
                tailwindcss -i ./templates/styles-in.css -o ./templates/styles.css
                ln -s ${chunkyFont} chunky.flf
                mkdir -p templates/assets/
                ln -s ${htmx_org} templates/assets/htmx.min.js
                ln -s ${htmx_ext_sse} templates/assets/sse.min.js
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
              tailwindcss_4
              rust-bin.stable.latest.default
              statix.packages.${pkgs.system}.default
              nom.packages.${pkgs.system}.default
            ];
          };
        };
      }
    );
}

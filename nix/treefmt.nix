{ ... }:
{
  projectRootFile = "flake.nix";
  settings.global.excludes = [
    ".sqlx/*"
    "justfile"
    "templates/logo.html"
    "Cargo.toml"
  ];
  programs.nixfmt.enable = true;
  programs.rustfmt.enable = true;
  programs.shfmt.enable = true;
  programs.sqlfluff = {
    enable = true;
    dialect = "sqlite";
  };
  programs.prettier.enable = true;
}

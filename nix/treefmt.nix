{ ... }:
{
  projectRootFile = "flake.nix";
  settings.global.excludes = [
    ".sqlx/*"
    "justfile"
    "templates/logo.html"
    "Cargo.toml"
    # sqlfluff is broken on this file for some reason
    "migrations/20250228190941_replies_table.sql"
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

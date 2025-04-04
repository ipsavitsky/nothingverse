_: {
  projectRootFile = "flake.nix";
  settings.global.excludes = [
    ".sqlx/*"
    "justfile"
    "templates/logo.html"
    "Cargo.toml"
    # sqlfluff is broken on this file for some reason
    "migrations/20250228190941_replies_table.sql"
    "migrations/20250402154446_add_generation_groups.sql"
  ];
  programs = {
    nixfmt.enable = true;
    rustfmt.enable = true;
    shfmt.enable = true;
    sqlfluff = {
      enable = true;
      dialect = "sqlite";
    };
    prettier.enable = true;
  };
}

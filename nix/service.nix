{ nothingverse }:
{
  lib,
  config,
  ...
}:
let
  cfg = config.services.nothingverse;
in
{
  options.services.nothingverse = {
    enable = lib.mkEnableOption "a social network where nohting happens";
    package = lib.mkOption {
      type = lib.types.package;
      inherit (nothingverse.${config.nixpkgs.system}) default;
    };

    ollama_url = lib.mkOption {
      type = lib.types.str;
    };

    ollama_port = lib.mkOption {
      type = lib.types.port;
    };

    model = lib.mkOption {
      type = lib.types.str;
      default = "nothingverse";
    };

    db_url = lib.mkOption {
      type = lib.types.path;
    };

    log_level = lib.mkOption {
      type = lib.types.str;
      default = "INFO";
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.nothingverse = {
      serviceConfig = {
        ExecStart = ''
          ${cfg.package}
            --ollama-url ${cfg.ollama_url}
            --ollama-port ${toString cfg.ollama_port}
            --model ${cfg.model}
            --db-url ${cfg.db_url}
            --log-level ${cfg.log_level}
        '';
      };
    };
  };
}

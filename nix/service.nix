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

    url = lib.mkOption {
      type = lib.types.str;
      default = "http://localhost:5000";
    };

    ollamaUrl = lib.mkOption {
      type = lib.types.str;
      default = "http://localhost:11434";
    };

    model = lib.mkOption {
      type = lib.types.str;
      default = "nothingverse";
    };

    dataDir = lib.mkOption {
      type = lib.types.path;
    };

    logLevel = lib.mkOption {
      type = lib.types.str;
      default = "INFO";
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.nothingverse = {
      serviceConfig = {
        ExecStart = ''
          ${cfg.package}
            --url ${cfg.url}
            --ollama-url ${cfg.ollamaUrl}
            --model ${cfg.model}
            --db-url ${cfg.dataDir}/nothing.sqlite
            --log-level ${cfg.logLevel}
        '';
      };
    };
  };
}

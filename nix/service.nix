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
      default = "/var/lib/nothingverse";
    };

    logLevel = lib.mkOption {
      type = lib.types.str;
      default = "INFO";
    };

    installOllamaModel = lib.mkOption {
      type = lib.types.bool;
      default = true;
    };

    group = lib.mkOption {
      type = lib.types.str;
      default = "nothingverse";
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = "nothingverse";
    };

    openFirewall = lib.mkOption {
      type = lib.types.bool;
      default = true;
    };
  };

  config = lib.mkIf cfg.enable {

    users.users = lib.mkIf (cfg.user == "nothingverse") {
      nothingverse = {
        group = cfg.group;
        isSystemUser = true;
      };
    };

    users.groups = lib.mkIf (cfg.group == "nothingverse") {
      nothingverse = { };
    };

    systemd.tmpfiles.rules = [
      "d '${cfg.dataDir}' 0700 ${cfg.user} ${cfg.group} - -"
    ];

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
        Restart = "on-failure";
        User = cfg.user;
        Group = cfg.group;
      };
    };

    preStart = lib.mkIf cfg.installOllamaModel ''
      ollama create nothing -f ${nothingverse.${config.nixpkgs.system}.modelfile}
    '';

    wantedBy = [ "multi-user.target" ];
  };
}

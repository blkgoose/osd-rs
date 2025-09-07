{
  description = "A Nix flake for osd-rs";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    nix-std.url = "github:chessai/nix-std";
  };
  outputs = { self, nixpkgs, flake-utils, nix-std }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      osd_bin = pkgs.rustPlatform.buildRustPackage {
        pname = "osd-rs";
        version = "0.1.0";
        src = self;
        cargoLock = { lockFile = ./Cargo.lock; };
      };

      map_conf = cfg: {
        watcher = let watchers = builtins.attrNames cfg;
        in builtins.map (name: { tag = name; } // (cfg."${name}")) watchers;
      };
      toTOML = cfg: nix-std.lib.serde.toTOML (map_conf cfg);
      conf_file = cfg: pkgs.writeText "osd-rs.toml" (toTOML cfg);
    in {
      homeManagerModules = {
        osd = { config, lib, ... }: {
          options.osd = {
            enable = lib.mkOption {
              type = lib.types.bool;
              default = false;
              description = "Enable the osd-rs service.";
            };

            settings = lib.mkOption {
              type = lib.types.attrsOf (lib.types.submodule
                ({ name, config, ... }: {
                  options = {
                    source = lib.mkOption {
                      type = lib.types.enum [ "file" "poll" ];
                      description =
                        "The source type for the watcher ('file' or 'poll').";
                    };

                    path = lib.mkOption {
                      type = lib.types.nullOr lib.types.str;
                      default = null;
                      description =
                        "Path to watch. Required if source is 'file'.";
                    };

                    command = lib.mkOption {
                      type = lib.types.nullOr lib.types.lines;
                      default = null;
                      description =
                        "Command to poll. Required if source is 'poll'.";
                    };

                    max = lib.mkOption {
                      type = lib.types.nullOr lib.types.int;
                      default = null;
                      description = "Optional maximum value for the watcher.";
                    };

                    min = lib.mkOption {
                      type = lib.types.nullOr lib.types.int;
                      default = null;
                      description = "Optional minimum value for the watcher.";
                    };

                    display_with = lib.mkOption {
                      type = lib.types.nullOr lib.types.str;
                      default = null;
                      description = "Optional system to display the value with";
                    };

                    interval = lib.mkOption {
                      type = lib.types.nullOr lib.types.int;
                      default = null;
                      description = "Polling interval in seconds.";
                    };

                    debug = lib.mkOption {
                      type = lib.types.bool;
                      default = false;
                      description = "Enable debug logging for this watcher.";
                    };
                  };
                }));

              default = { };
              example = {
                brightness = {
                  source = "file";
                  path = "/sys/class/backlight/intel_backlight/brightness";
                  max = 19393;
                };
                volume = {
                  source = "poll";
                  command = "wpctl get-volume @DEFAULT_AUDIO_SINK@";
                };
              };
              description = "Declarative definition of watchers.";
            };
          };

          config = lib.mkIf config.osd.enable {
            assertions = lib.flatten (lib.mapAttrsToList (name: watcher:
              (lib.optionals (watcher.source == "file") [{
                assertion = watcher.path != null;
                message =
                  "Watcher '${name}' is defined as 'file' and thus requires a 'path' to be specified.";
              }]) ++ (lib.optionals (watcher.source == "poll") [{
                assertion = watcher.command != null;
                message =
                  "Watcher '${name}' is defined as 'poll' and thus requires a 'command' to be specified.";
              }])) config.osd.settings);

            systemd.user.services.osd = {
              Unit.Description = "Runs osd-rs";
              Service.ExecStart =
                "${osd_bin}/bin/osd-rs ${conf_file config.osd.settings}";
              Install.WantedBy = [ "graphical-session.target" ];

              Unit = {
                After = "graphical-session.target";
                PartOf = "graphical-session.target";
              };
            };
          };
        };
      };
    };
}

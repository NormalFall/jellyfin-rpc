{
  description = "Flake for jellyfin-rpc";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
  let
    system = "x86_64-linux";
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
  in {
    packages."${system}".default = pkgs.callPackage ./default.nix {};

    devShells."${system}".default = pkgs.mkShell {
      buildInputs = [ (pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" ];
      }) ];
    };

    overlay = self.outputs.overlays.default;
    overlays.default = final: prev: import ./overlay.nix {inherit final prev;};

    homeManagerModule = self.outputs.homeManagerModules.default;
    homeManagerModules.default = {
      pkgs,
      config,
      lib,
      ...
    }: let
      cfg = config.services.jellyfin-rpc;
    in
    with lib; {
      options.services.jellyfin-rpc = {
        enable = mkEnableOption "Enables the systemd module for jellyfin-rpc";
        configPath = mkOption {
          default = "${config.home.homeDirectory}/.config/jellyfin-rpc/main.json";
          example = "/my/path/conf.json";
          description = "Location of the main config file";
        };

        config = mkOption {
          default = {};
          example = {
            jellyfin = {
              url = "https://example.com";
              api_key = "sadasodsapasdskd";
              username = "your_username_here";
              self_signed_cert = false;
              show_simple = false;
            };

            discord = {
              application_id = "1053747938519679018";
              show_paused = true;
            };

            imgbb = {
              api_token = "asdjdjdg394209fdjs093";
              expiration = 432000;
            };

            images = {
              enable_images = true;
              imgbb_images = true;
            };
          };
          description = "Jellyfin-RPC main config";
        };

        jellyfinSecretPath = mkOption {
          default = "";
          example = "${config.xdg.configHome}/jellyfin-rpc/jellyfin.key";
          description = ''
          If empty it will default to being disabled.
          Sets where the jellyfin api key file is stored.
          This option is usefull if you don't want your keys stored inside of you nix config
          '';
        };

        imgbbSecretPath = mkOption {
          default = "";
          example = "${config.xdg.configHome}/jellyfin-rpc/imgbb.key";
          description = ''
          If empty it will default to being disabled.
          Sets where the imgbb api key file is stored.
          This option is usefull if you don't want your keys stored inside of you nix config
          '';
        };
      };

      config = mkIf cfg.enable {
        nixpkgs.overlays = [ self.outputs.overlay ];

        xdg.dataFile."${cfg.configPath}".source = let
          jsonFormat = pkgs.formats.json {};
        in jsonFormat.generate "jellyfin-rpc-config" cfg.config;

        systemd.user.services.jellyfin-rpc = {
          Unit = {
            Description = "Displays the content on jellyfin you're currently watching on Discord.";
          };

          Install = {
            WantedBy = [ "default.target" ];
          };

          Service = {
            ExecStart = "${pkgs.jellyfin-rpc}/bin/jellyfin-rpc -c ${cfg.configPath}"
              + (if (cfg.jellyfinSecretPath == "") then "" else " -j ${cfg.jellyfinSecretPath}")
              + (if (cfg.imgbbSecretPath == "") then "" else " -a ${cfg.imgbbSecretPath}");
          };
        };
      };
    };
  };
}
{ config, ... }: {
  # from ../../crates/prime-domain/src/migrate.rs
  test-data = {
    archive = ../../crates/owl/test/ky2wzr68im63ibgzksbsar19iyk861x6-bat-0.25.0;
    store-path = "ky2wzr68im63ibgzksbsar19iyk861x6-bat-0.25.0";
    deriver = "4yz8qa58nmysad5w88rgdhq15rkssqr6-bat-0.25.0";
    deriver-system = "aarch64-linux";
    email = "jpicard@federation.gov";
    password = "password";
    cache = "aaron";
    store = "albert";
  };
  
  grid-node = {
    networking.firewall.allowedTCPPorts = [ 3000 ];

    systemd.services.grid = {
      description = "Grid Server";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        Type = "simple";
        ExecStart = "${config.packages.grid}/bin/grid --migrate --host 0.0.0.0 --no-secure-cookies";
      };
    };
  };

  client-node = { pkgs, ... }: {
    environment.systemPackages = with pkgs; [
      curl jq
      config.packages.cli
    ];
    nix.extraOptions = "experimental-features = nix-command";
  };
}

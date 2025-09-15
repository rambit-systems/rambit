{ config, pkgs, ... }: {
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
        ExecStartPost = pkgs.writeShellScript "grid-health-check" ''
          timeout=20
          interval=1
          elapsed=0
          
          # echo "Waiting for grid service to become healthy..."
          while [ $elapsed -lt $timeout ]; do
            if ${pkgs.curl}/bin/curl -s -f http://localhost:3000/api/v1/health >/dev/null 2>&1; then
              # echo "Grid service is healthy!"
              exit 0
            fi
            # echo "Health check failed, retrying in $interval seconds... ($elapsed/$timeout)"
            sleep $interval
            elapsed=$((elapsed + interval))
          done
          
          echo "Grid service failed to become healthy within $timeout seconds"
          exit 1
        '';
      };

      environment = {
        POSTGRES_URL = "postgresql://grid:grid_password@localhost:5432/grid";
      };
    };
  };

  client-node = { pkgs, ... }: {
    environment.systemPackages = with pkgs; [
      curl jq
      config.packages.junk-cli
    ];
    nix.extraOptions = "experimental-features = nix-command";
  };
}

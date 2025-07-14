{ pkgs, config, ... }: let
  # from ../../../crates/prime-domain/src/migrate.rs
  archive = ../../../crates/owl/test/ky2wzr68im63ibgzksbsar19iyk861x6-bat-0.25.0;
  store-path = "ky2wzr68im63ibgzksbsar19iyk861x6-bat-0.25.0";
  deriver = "4yz8qa58nmysad5w88rgdhq15rkssqr6-bat-0.25.0";
  deriver-system = "aarch64-linux";
  user-id = "01JXGXV4R6VCZWQ2DAYDWR1VXD";
  cache = "aaron";
  store = "albert";

  grid-node = {
    networking.firewall.allowedTCPPorts = [ 3000 ];

    systemd.services.grid = {
      description = "grid Server";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        Type = "simple";
        ExecStart = "${config.packages.grid}/bin/grid --migrate --host 0.0.0.0";
      };
    };
  };
  client-node = { pkgs, ... }: {
    environment.systemPackages = with pkgs; [
      curl jq
      config.packages.cli
    ];
  };
in {
  domain-api-upload-download-curl = pkgs.testers.runNixOSTest {
    name = "domain-api-upload-download-curl";

    nodes = {
      grid = grid-node;
      client = client-node;
    };

    testScript = ''
      start_all()

      grid.wait_for_unit("grid.service")

      client.wait_for_unit("network.target")
      client.succeed("ping -c 1 grid")

      client.succeed("curl -X POST http://grid:3000/upload \
        --url-query caches=${cache} \
        --url-query store_path=${store-path} \
        --url-query target_store=${store} \
        --url-query deriver_store_path=${deriver} \
        --url-query deriver_system=${deriver-system} \
        -H 'x-user-id: ${user-id}' \
        --data-binary @${archive} \
      ")

      client.succeed("curl http://grid:3000/download/${cache}/${store-path} > output")
      client.succeed("diff ${archive} output")
    '';
  };

  domain-api-upload-download-cli = pkgs.testers.runNixOSTest {
    name = "domain-api-upload-download-cli";

    nodes = {
      grid = grid-node;
      client = client-node;
    };

    testScript = ''
      start_all()

      grid.wait_for_unit("grid.service")

      client.wait_for_unit("network.target")
      client.succeed("ping -c 1 grid")

      client.succeed("${config.packages.cli}/bin/cli \
        --host grid \
        upload \
        --caches ${cache} \
        --store-path ${store-path} \
        --target-store ${store} \
        --deriver-system ${deriver-system} \
        --deriver-store-path ${deriver} \
        --user ${user-id} \
        --nar ${archive} \
      ")

      client.succeed("curl http://grid:3000/download/${cache}/${store-path} > output")
      client.succeed("diff ${archive} output")
    '';
  };
}

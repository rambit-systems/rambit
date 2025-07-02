{ pkgs, config, ... }: let
  # from ../../../crates/prime-domain/src/migrate.rs
  user-id = "01JXGXV4R6VCZWQ2DAYDWR1VXD";
  cache = "aaron";
  path = "foo";
  store = "albert";

  app-node = {
    networking.firewall.allowedTCPPorts = [ 3000 ];

    systemd.services.app = {
      description = "App Server";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        Type = "simple";
        ExecStart = "${config.packages.app}/bin/app --migrate --host 0.0.0.0";
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
      app = app-node;
      client = client-node;
    };

    testScript = ''
      start_all()

      app.wait_for_unit("app.service")

      client.wait_for_unit("network.target")
      client.succeed("ping -c 1 app")

      client.succeed("curl -X POST \
        http://app:3000/upload/${cache}/${path}/${store} \
        -H 'user_id: ${user-id}' \
        -d @${./upload-download.nix} \
      ")

      client.succeed("curl http://app:3000/download/${cache}/${path}")
    '';
  };

  domain-api-upload-download-cli = pkgs.testers.runNixOSTest {
    name = "domain-api-upload-download-cli";

    nodes = {
      app = app-node;
      client = client-node;
    };

    testScript = ''
      start_all()

      app.wait_for_unit("app.service")

      client.wait_for_unit("network.target")
      client.succeed("ping -c 1 app")

      client.succeed("${config.packages.cli}/bin/cli \
        --host app \
        upload \
        --cache ${cache} \
        --entry-path ${path} \
        --store ${store} \
        --user ${user-id} \
        --file ${./upload-download.nix} \
      ")

      client.succeed("curl http://app:3000/download/${cache}/${path}")
    '';
  };
}

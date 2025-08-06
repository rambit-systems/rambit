{ pkgs, config, common, ... }: let
  inherit (common) test-data grid-node client-node;
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

      # make sure the grid node is reachable
      client.wait_for_unit("network.target")
      client.succeed("ping -c 1 grid")

      # authenticate and hold onto the headers
      client.succeed("curl --fail-with-body -X POST http://grid:3000/api/v1/authenticate \
        -b cookie.txt -c cookie.txt \
        -H \"Content-Type: application/json\" \
        -d '{\"email\":\"${test-data.email}\",\"password\":\"${test-data.password}\"}' \
      ")

      # upload
      client.succeed("curl --fail-with-body -X POST http://grid:3000/api/v1/upload \
        -b cookie.txt -c cookie.txt \
        --url-query caches=${test-data.cache} \
        --url-query store_path=${test-data.store-path} \
        --url-query target_store=${test-data.store} \
        --url-query deriver_store_path=${test-data.deriver} \
        --url-query deriver_system=${test-data.deriver-system} \
        --data-binary @${test-data.archive} \
      ")

      # download the payload
      client.succeed("curl --fail-with-body http://grid:3000/api/v1/c/${test-data.cache}/download/${test-data.store-path} > output")
      # make sure it's byte-identical
      client.succeed("diff ${test-data.archive} output")

      # make sure nix can access it as a binary cache path
      # with the --no-trust flag because we're not signing yet
      client.succeed("nix store verify --no-trust --store http://grid:3000/api/v1/c/${test-data.cache} /nix/store/${test-data.store-path}")
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

      # make sure the grid node is reachable
      client.wait_for_unit("network.target")
      client.succeed("ping -c 1 grid")

      # upload
      client.succeed("cli \
        --host http://grid \
        --email ${test-data.email} \
        --password ${test-data.password} \
        upload \
        --caches ${test-data.cache} \
        --store-path ${test-data.store-path} \
        --target-store ${test-data.store} \
        --deriver-system ${test-data.deriver-system} \
        --deriver-store-path ${test-data.deriver} \
        --nar ${test-data.archive} \
      ")

      # download the payload
      client.succeed("curl --fail-with-body http://grid:3000/api/v1/c/${test-data.cache}/download/${test-data.store-path} > output")
      # make sure it's byte-identical
      client.succeed("diff ${test-data.archive} output")

      # make sure nix can access it as a binary cache path
      # with the --no-trust flag because we're not signing yet
      client.succeed("nix store verify --no-trust --store http://grid:3000/api/v1/c/${test-data.cache} /nix/store/${test-data.store-path}")
    '';
  };
}

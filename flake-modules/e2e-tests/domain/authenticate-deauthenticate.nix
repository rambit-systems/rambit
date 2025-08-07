{ pkgs, common, ... }: let
  inherit (common) test-data grid-node client-node;
in {
  domain-api-authenticate-deauthenticate-curl = pkgs.testers.runNixOSTest { 
    name = "domain-api-authenticate-deauthenticate-cli";

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

      # authenticate and hold onto the cookies
      client.succeed("curl --fail-with-body -X POST http://grid:3000/api/v1/authenticate \
        -b cookie.txt -c cookie.txt \
        -H \"Content-Type: application/json\" \
        -d '{\"email\":\"${test-data.email}\",\"password\":\"${test-data.password}\"}' \
      ")

      # extract "id" cookie and assert that it's NON-EMPTY
      client.succeed("session_id=$(awk '/id/ {print $7}' cookie.txt) \
        && [[ -n \"$session_id\" ]] || { exit 1; }")

      # deauthenticate with same cookie jar
      client.succeed("curl --fail-with-body -X POST http://grid:3000/api/v1/deauthenticate \
        -b cookie.txt -c cookie.txt \
      ")

      # extract "id" cookie and assert that it's EMPTY
      client.succeed("session_id=$(awk '/id/ {print $7}' cookie.txt) \
        && [[ -z \"$session_id\" ]] || { exit 1; }")
    '';
  };
}

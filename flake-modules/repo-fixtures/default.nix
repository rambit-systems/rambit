{ ... }: {
  perSystem = { pkgs, ... }: let
    quickwit-config = {
      version = "0.7";

      data_dir = "/tmp/rambit-qwdata/";
    };

    quickwit-config-yaml = pkgs.lib.generators.toYAML { } quickwit-config;
    quickwit-config-file = pkgs.writeText "quickwit-config.yaml" quickwit-config-yaml;
  in {
    config._module.args = {
      quickwit-config = quickwit-config-file;
    };
  };
}

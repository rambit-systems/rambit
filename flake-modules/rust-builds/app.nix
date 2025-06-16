{ ... }: {
  perSystem = { pkgs, rust-toolchain, rust-workspace, ... }: let
    inherit (rust-toolchain) craneLib;
    inherit (rust-workspace) workspace-args;

    app = craneLib.buildPackage (workspace-args // {
      pname = "app";
    });

    app-image = pkgs.dockerTools.buildLayeredImage {
      name = "app";
      tag = "latest";
      config = {
        Entrypoint = [ "${app}/bin/app" "--" ];
        WorkingDir = "${app}/bin";
      };
    };
  in {
    packages = {
      inherit app app-image;
      default = app;
    };
  };
}

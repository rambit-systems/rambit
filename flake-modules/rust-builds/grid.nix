{ ... }: {
  perSystem = { pkgs, rust-toolchain, rust-workspace, ... }: let
    inherit (rust-toolchain) craneLib;
    inherit (rust-workspace) workspace-args;

    grid = craneLib.buildPackage (workspace-args // {
      pname = "grid";
      cargoExtraArgs = "-p grid";
    });

    grid-image = pkgs.dockerTools.buildLayeredImage {
      name = "grid";
      tag = "latest";
      config = {
        Entrypoint = [ "${grid}/bin/grid" "--" ];
        WorkingDir = "${grid}/bin";
      };
    };
  in {
    packages = {
      inherit grid grid-image;
      default = grid;
    };
  };
}

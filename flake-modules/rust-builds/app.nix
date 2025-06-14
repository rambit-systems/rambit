{ ... }: {
  perSystem = { pkgs, rust-toolchain, rust-workspace, ... }: let
    inherit (rust-toolchain) craneLib;
    inherit (rust-workspace) workspace-args;

    app = craneLib.buildPackage (workspace-args // {
      pname = "app";
    });
  in {
    packages = {
      inherit app;
    };
  };
}

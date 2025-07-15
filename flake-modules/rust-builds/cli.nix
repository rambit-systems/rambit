{ ... }: {
  perSystem = { pkgs, rust-toolchain, rust-workspace, ... }: let
    inherit (rust-toolchain) craneLib;
    inherit (rust-workspace) workspace-args;

    cli = craneLib.buildPackage (workspace-args // {
      pname = "cli";
      cargoExtraArgs = "-p cli";
    });
  in {
    packages = {
      inherit cli;
    };
  };
}

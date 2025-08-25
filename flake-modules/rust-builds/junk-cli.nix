{ ... }: {
  perSystem = { pkgs, rust-toolchain, rust-workspace, ... }: let
    inherit (rust-toolchain) craneLib;
    inherit (rust-workspace) workspace-args;

    junk-cli = craneLib.buildPackage (workspace-args // {
      pname = "junk-cli";
      cargoExtraArgs = "-p junk-cli";
    });
  in {
    packages = {
      inherit junk-cli;
    };
  };
}

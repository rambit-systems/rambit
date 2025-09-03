{ ... }: {
  perSystem = { pkgs, rust-toolchain, rust-workspace, ... }: let
    inherit (rust-toolchain) craneLib;
    inherit (rust-workspace) workspace-args;

    junk-cli = craneLib.buildPackage (workspace-args // {
      pname = "junk-cli";
      cargoExtraArgs = "-p junk-cli";

      CARGO_BUILD_TARGET = rust-toolchain.musl-target;
      CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
    });
  in {
    packages = {
      inherit junk-cli;
    };
  };
}

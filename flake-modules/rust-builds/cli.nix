{ ... }: {
  perSystem = { pkgs, rust-toolchain, rust-workspace, ... }: let
    inherit (rust-toolchain) craneLib;
    inherit (rust-workspace) workspace-args;

    cli = craneLib.buildPackage (workspace-args // {
      pname = "cli";
      cargoExtraArgs = "-p cli";

      CARGO_BUILD_TARGET = rust-toolchain.musl-target;
      CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
    });
  in {
    packages = {
      inherit cli;
    };
  };
}

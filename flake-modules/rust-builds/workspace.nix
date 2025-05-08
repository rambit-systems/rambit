{ inputs, ... }: {
  perSystem = { pkgs, rust-toolchain, ... }: let
    filter = inputs.nix-filter.lib;

    # configure the source
    src = filter {
      root = ../../.; # project root
      include = [
        "crates" "Cargo.toml" "Cargo.lock" # typical rust source
        ".cargo"                           # extra rust config
        (filter.matchExt "toml")           # extra toml used by other projects
        "media"                            # static assets
      ];
    };

    # build arguments for the whole workspace
    workspace-base-args = {
      inherit src;
      strictDeps = true;

      pname = "rambit";
      version = "0.1";
      doCheck = false;

      # inputs assumed to be relevant for all crates
      nativeBuildInputs = with pkgs; [
        pkg-config gcc
      ];
      buildInputs = [ ];
    };

    # build the deps for the whole workspace
    workspace-base-cargo-artifacts = rust-toolchain.craneLib.buildDepsOnly workspace-base-args;
  in {
    # pass back to the flake
    config._module.args.rust-workspace = {
      inherit workspace-base-args workspace-base-cargo-artifacts;
    };
    config.checks = let
      args-with-artifacts = workspace-base-args // { cargoArtifacts = workspace-base-cargo-artifacts; };
    in {
      # run clippy, denying warnings
      rust-cargo-clippy = rust-toolchain.craneLib.cargoClippy (args-with-artifacts // {
        cargoClippyExtraArgs = "--all-targets --no-deps -- --deny warnings";
        pnameSuffix = "-clippy-all";
      });
      # run rust-doc, denying warnings
      rust-cargo-docs = rust-toolchain.craneLib.cargoDoc (args-with-artifacts // {
        cargoClippyExtraArgs = "--no-deps";
        RUSTDOCFLAGS = "-D warnings";
      });
      # run rust tests with nextest
      rust-cargo-nextest = rust-toolchain.craneLib.cargoNextest (args-with-artifacts // {
        partitions = 1;
        partitionType = "count";
      });
      # run rust doc tests
      rust-cargo-doctests = rust-toolchain.craneLib.cargoDocTest args-with-artifacts;
      # run cargo fmt, failing if not already formatted perfectly
      rust-cargo-fmt = rust-toolchain.craneLib.cargoFmt workspace-base-args;
      # run taplo fmt, failing if not already formatted perfectly
      rust-toml-fmt = rust-toolchain.craneLib.taploFmt workspace-base-args;
    };
  };
}


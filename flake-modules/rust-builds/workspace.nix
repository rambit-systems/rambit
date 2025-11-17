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

      # inputs assumed to be relevant for all crates
      nativeBuildInputs = with pkgs; [
        pkg-config clang mold
      ];
      buildInputs = [ ];
    };

    # build the deps for the whole workspace
    workspace-cargo-artifacts = rust-toolchain.craneLib.buildDepsOnly workspace-base-args;

    # feed it back into the args
    workspace-args = workspace-base-args // {
      cargoArtifacts = workspace-cargo-artifacts;
    };
  in {
    # pass back to the flake
    config._module.args.rust-workspace = {
      inherit workspace-base-args workspace-args workspace-cargo-artifacts;
    };
    config.checks = {
      # run clippy, denying warnings
      rust-cargo-clippy = rust-toolchain.craneLib.cargoClippy (workspace-args // {
        cargoClippyExtraArgs = "--all-targets --no-deps -- --deny warnings";
        pnameSuffix = "-clippy-all";
      });
      # run rust-doc, denying warnings
      rust-cargo-docs = rust-toolchain.craneLib.cargoDoc (workspace-args // {
        cargoClippyExtraArgs = "--no-deps";
        RUSTDOCFLAGS = "-D warnings";
      });
      # run rust tests with nextest
      rust-cargo-nextest = rust-toolchain.craneLib.cargoNextest (workspace-args // {
        partitions = 1;
        partitionType = "count";
      });
      # run rust doc tests
      rust-cargo-doctests = rust-toolchain.craneLib.cargoDocTest workspace-args;
      # run cargo fmt, failing if not already formatted perfectly
      rust-cargo-fmt = rust-toolchain.craneLib.cargoFmt workspace-base-args;
      # run taplo fmt, failing if not already formatted perfectly
      rust-toml-fmt = rust-toolchain.craneLib.taploFmt workspace-base-args;
    };
  };
}


{ inputs, ... }: {
  perSystem = { pkgs, ... }: let
    filter = inputs.nix-filter.lib;

    # configure the source
    src = filter {
      root = ../../.;
      include = [
        "crates" "Cargo.toml" "Cargo.lock" # typical rust source
        ".cargo" # extra rust config
        (filter.matchExt "toml") # extra toml used by other projects
        "media" # static assets
      ];
    };

    # build the CI and dev toolchains
    toolchain = p: p.rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal.override {
      extensions = [ "rustfmt" "clippy" ];
    });
    dev-toolchain = p: p.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
      extensions = [ "rust-src" "rust-analyzer" ];
      targets = [ "wasm32-unknown-unknown" ];
    });

    # configure crane to use the CI toolchain
    craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;

    # arguments shared by all rust packages we build
    common-args = {
      inherit src;
      strictDeps = true;

      pname = "rambit";
      version = "0.1";
      doCheck = false;

      nativeBuildInputs = with pkgs; [
        pkg-config
      ];
      buildInputs = with pkgs; [
        openssl
      ];
    };

    # build the deps for the whole workspace
    cargoArtifacts = craneLib.buildDepsOnly common-args;

    # builder functions for individual crates    
    individual-crate-args = crate-name: common-args // {
      inherit cargoArtifacts;
      pname = crate-name;
      cargoExtraArgs = "-p ${crate-name}";
    };
    build-crate = name: craneLib.buildPackage (individual-crate-args name);

    crate-graph = craneLib.mkCargoDerivation {
      inherit src;
      cargoArtifacts = null;
      pname = "crate-graph";
      version = "0.1";
      buildPhaseCargoCommand = ''
        cargo depgraph --workspace-only > crate-graph.dot
      '';
      installPhaseCommand = ''
        mkdir $out
        cp crate-graph.dot $out
      '';
      doInstallCargoArtifacts = false;
      nativeBuildInputs = with pkgs; [ cargo-depgraph ];
    };

    crate-graph-image = pkgs.stdenv.mkDerivation {
      inherit src;
      cargoArtifacts = null;
      pname = "crate-graph-image";
      version = "0.1";
      buildPhase = ''
        export XDG_CACHE_HOME="$(mktemp -d)"
        dot -Tsvg ${crate-graph}/crate-graph.dot > crate-graph.svg
      '';
      installPhase = ''
        mkdir $out
        cp crate-graph.svg $out
      '';
      FONTCONFIG_FILE = pkgs.makeFontsConf {
        fontDirectories = [ pkgs.dejavu_fonts ];
      };
      doInstallCargoArtifacts = false;
      nativeBuildInputs = with pkgs; [ graphviz cargo-depgraph ];
    };
  in {
    packages = {
      api = build-crate "api";
      daemon = build-crate "daemon";
      migrator = build-crate "migrator";
      toolchain = toolchain pkgs;
      dev-toolchain = dev-toolchain pkgs;
      inherit crate-graph crate-graph-image;
    };
    checks = {
      # run clippy, denying warnings
      rust-cargo-clippy = craneLib.cargoClippy (common-args // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets --no-deps -- --deny warnings";
      });
      # run rust-doc, denying warnings
      rust-cargo-docs = craneLib.cargoDoc (common-args // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--no-deps";
        RUSTDOCFLAGS = "-D warnings";
      });
      # run rust tests with nextest
      rust-cargo-nextest = craneLib.cargoNextest (common-args // {
        inherit cargoArtifacts;
        partitions = 1;
        partitionType = "count";
      });
      # run cargo fmt, failing if not already formatted perfectly
      rust-cargo-fmt = craneLib.cargoFmt common-args;
      # run cargo deny, failing if anything gets flagged
      rust-cargo-deny = craneLib.cargoDeny common-args;
    };
  };
}

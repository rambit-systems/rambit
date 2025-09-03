{ ... }: {
  perSystem = { pkgs, rust-toolchain, rust-workspace, system, ... }: let
    inherit (rust-workspace.workspace-base-args) src;
    inherit (rust-workspace) workspace-cargo-artifacts;
    inherit (rust-toolchain) craneLib;

    # get the leptos options from the Cargo.toml
    workspace-cargo-manifest = builtins.fromTOML (builtins.readFile ../../Cargo.toml);
    leptos-options = builtins.elemAt workspace-cargo-manifest.workspace.metadata.leptos 0;

    # get the style node_modules for the frontend
    js2nix = pkgs.callPackage (pkgs.fetchgit {
      url = "https://github.com/canva-public/js2nix";
      hash = "sha256-udsxrWLtAaBkh++pqal3u5+hI0YhWI06O2UaC6IS5lY=";
    }) { };
    style-root = ../../crates/site-app/style;
    style-node-env = (js2nix {
      package-json = style-root + "/package.json";
      yarn-lock = style-root + "/yarn.lock";
    }).nodeModules;

    # options for both the frontend and server builds
    common-args = {
      inherit src;
      pname = leptos-options.bin-package;
      version = "0.1.0";

      doCheck = false;

      nativeBuildInputs = (with pkgs; [
        pkg-config
        binaryen # provides wasm-opt for cargo-leptos
        clang lld mold
      ]) ++ pkgs.lib.optionals (system == "x86_64-linux") [
        pkgs.nasm # wasm compiler only for x86_64-linux
      ];
      buildInputs = [ ];
    };

    # build the deps for the frontend bundle, and export the target folder
    frontend-deps = craneLib.mkCargoDerivation (common-args // {
      pname = "${leptos-options.lib-package}-deps";
      src = craneLib.mkDummySrc common-args;
      cargoArtifacts = workspace-cargo-artifacts;
      doInstallCargoArtifacts = true;

      buildPhaseCargoCommand = ''
        cargo build \
          --package=${leptos-options.lib-package} \
          --lib \
          --target-dir=/build/source/target/front \
          --target=wasm32-unknown-unknown \
          --no-default-features \
          --profile=${leptos-options.lib-profile-release}
      '';
    });

    # build the deps for the server binary, and export the target folder
    server-deps = craneLib.mkCargoDerivation (common-args // {
      pname = "${leptos-options.bin-package}-deps";
      src = craneLib.mkDummySrc common-args;
      cargoArtifacts = frontend-deps;
      doInstallCargoArtifacts = true;

      buildPhaseCargoCommand = ''
        cargo build \
          --package=${leptos-options.bin-package} \
          --no-default-features \
          --features json-tracing \
          --release
      '';
    });

    # build the binary and bundle using cargo leptos
    server = craneLib.buildPackage (common-args // {
      # add inputs needed for leptos build
      nativeBuildInputs = common-args.nativeBuildInputs ++ (with pkgs; [
        cargo-leptos tailwindcss_4 makeWrapper
     ]);

      # link the style packages node_modules into the build directory
      preBuild = ''
        ln -s ${style-node-env} \
          ./crates/site-app/style/node_modules
      '';
      
      # enable hash_files again, so we generate `hash.txt`
      buildPhaseCargoCommand = ''
        LEPTOS_HASH_FILES=true cargo leptos build --bin-features json-tracing --release -vvv
      '';
      doNotPostBuildInstallCargoBinaries = true;

      installPhaseCommand = ''
        mkdir -p $out/bin
        cp target/release/${leptos-options.bin-package} $out/bin/
        cp target/release/hash.txt $out/bin/
        cp -r target/site $out/bin/

        # supply env variable defaults from leptos options
        wrapProgram $out/bin/${leptos-options.bin-package} \
          --set-default LEPTOS_OUTPUT_NAME ${leptos-options.name} \
          --set-default LEPTOS_SITE_ROOT $out/bin/${leptos-options.name} \
          --set-default LEPTOS_SITE_PKG_DIR ${leptos-options.site-pkg-dir} \
          --set-default LEPTOS_SITE_ADDR 0.0.0.0:3000 \
          --set-default LEPTOS_RELOAD_PORT ${builtins.toString leptos-options.reload-port} \
          --set-default LEPTOS_ENV PROD \
          --set-default LEPTOS_HASH_FILES true
      '';

      doCheck = false;
      cargoArtifacts = server-deps;
    });

    server-container = pkgs.dockerTools.buildLayeredImage {
      name = leptos-options.bin-package;
      tag = "latest";
      contents = [
        server
        pkgs.cacert
        pkgs.bash
      ];
      config = {
        # runs the executable with tini: https://github.com/krallin/tini
        # this does signal forwarding and zombie process reaping
        # this should be removed if using something like firecracker (i.e. on fly.io)
        # Entrypoint = [ "${pkgs.tini}/bin/tini" "${leptos-options.bin-package}" "--" ];
        Entrypoint = [ "${leptos-options.bin-package}" ];
        WorkingDir = "${server}/bin";
      };
    };
  in {
    packages = {
      default = server;
      "${leptos-options.bin-package}" = server;
      "${leptos-options.bin-package}-container" = server-container;
    };
  };
}


{ ... }: {
  perSystem = { pkgs, rust-toolchain, rust-workspace, system, ... }: let
    inherit (rust-workspace) workspace-cargo-artifacts workspace-base-args;
    inherit (workspace-base-args) src;
    inherit (rust-toolchain) craneLib;

    # get the style node_modules for tailwind
    js2nix = pkgs.callPackage (pkgs.fetchgit {
      url = "https://github.com/canva-public/js2nix";
      hash = "sha256-udsxrWLtAaBkh++pqal3u5+hI0YhWI06O2UaC6IS5lY=";
    }) { };
    style-root = ../../crates/app/style;
    style-node-env = (js2nix {
      package-json = style-root + "/package.json";
      yarn-lock = style-root + "/yarn.lock";
    }).nodeModules;

    # transform the css with tailwind
    css = pkgs.stdenv.mkDerivation {
      pname = "grid-css";
      version = "0.1.0";
      inherit src;

      buildPhase = ''
        cd crates/app/style/

        ln -s -T ${style-node-env} node_modules

        ${pkgs.tailwindcss_4}/bin/tailwindcss \
          --input src/main.css \
          --output $out \
          --minify
      '';
    };

    server = craneLib.buildPackage (workspace-base-args // {
      pname = "grid";

      cargoArtifacts = workspace-cargo-artifacts;
      nativeBuildInputs = workspace-base-args.nativeBuildInputs ++ (with pkgs; [
        makeWrapper
      ]);

      doNotPostBuildInstallCargoBinaries = true;
      installPhaseCommand = ''
        mkdir -p $out/bin
        cp target/release/grid $out/bin/grid
        cp ${css} $out/bin/styles.css
        cp -r crates/app/public $out/bin/public

        # supply env variable defaults from leptos options
        wrapProgram $out/bin/grid \
          --set-default GRID_ENV prod \
          --set-default GRID_STATIC_ASSET_DIR $out/bin/public \
          --set-default GRID_STYLESHEET_PATH $out/bin/styles.css \
      '';

      doCheck = false;
    });

    server-container = pkgs.dockerTools.buildLayeredImage {
      name = "grid";
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
        Entrypoint = [ "${pkgs.tini}/bin/tini" "grid" "--" ];
        # Entrypoint = [ "grid" ];
        WorkingDir = "${server}/bin";
      };
    };
  in {
    packages = {
      default = server;
      grid = server;
      grid-container = server-container;
    };
  };
}


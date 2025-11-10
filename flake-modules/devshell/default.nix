{ ... }: {
  perSystem = { pkgs, inputs', config, rust-toolchain, ... }: {
    devShells.default = let
      libraries = with pkgs; [
        openssl
      ];
      packages = with pkgs; [
        (rust-toolchain.dev-toolchain pkgs)

        # libraries used in local rust builds
        pkg-config

        # other things used in local rust builds
        clang mold

        # cargo tools
        cargo-nextest # testing
        cargo-deny # package auditing
        cargo-depgraph # dependency graphing

        # leptos items
        cargo-leptos binaryen # leptos build tools
        tailwindcss_4 yarn # css build tools

        # deployment
        dive flyctl
      ];

      # this is just so we can extract $PKG_CONFIG_PATH
      libshell = pkgs.stdenv.mkDerivation {
        src = ./.;
        name = "rust-libraries-shell";
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = libraries;
        buildPhase = ''
          echo $PKG_CONFIG_PATH > $out
        '';
      };
      PKG_CONFIG_PATH = builtins.readFile libshell;
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
    in pkgs.devshell.mkShell {
      inherit packages;

      motd = "\n  Welcome to the {2}rambit{reset} dev shell. Run {1}menu{reset} for commands.\n";

      env = [
        {
          name = "PKG_CONFIG_PATH";
          prefix = PKG_CONFIG_PATH;
        }
        {
          name = "LD_LIBRARY_PATH";
          prefix = LD_LIBRARY_PATH;
        }
      ];

      commands = [
        {
          name = "check";
          command = "nix flake check -L";
          help = "Run nix checks";
          category = "[nix actions]";
        }

        {
          name = "container";
          command = ''
            docker load -i $(nix build .#grid-container --print-out-paths --no-link) && \
            docker run \
              --rm --network host \
              -e POSTGRES_URL='postgresql://postgres:password@localhost:6432/main' \
              grid:latest
          '';
          help = "Runs the site binary in a container.";
        }

        {
          name = "db";
          command = ''
            docker run --rm -e POSTGRES_DB=main -e POSTGRES_PASSWORD=password -p 6432:5432 $@ postgres
          '';
          help = "Runs PostgreSQL in a container.";
        }

        {
          name = "update-crate-graph";
          command = ''
            echo "building crate graph image"
            CRATE_GRAPH_IMAGE_PATH=$(nix build .#crate-graph-image --print-out-paths --no-link)
            echo "updating crate graph image in repo ($PRJ_ROOT/media/crate-graph.svg)"
            cp $CRATE_GRAPH_IMAGE_PATH/crate-graph.svg $PRJ_ROOT/media/crate-graph.svg --no-preserve=mode
          '';
          help = "Update the crate graph";
          category = "[repo actions]";
        }
      ];
    };
  };
}

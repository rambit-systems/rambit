{ ... }: {
  perSystem = { pkgs, inputs', config, rust-toolchain, ... }: {
    devShells.default = pkgs.devshell.mkShell {
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
        dart-sass tailwindcss_4 yarn # css build tools
      ];

      motd = "\n  Welcome to the {2}rambit{reset} dev shell. Run {1}menu{reset} for commands.\n";

      commands = [
        {
          name = "test";
          command = "cargo nextest run";
          help = "Run tests with nextest";
          category = "[testing]";
        }
        {
          name = "test-all";
          command = "cargo nextest run --run-ignored all";
          help = "Run all tests, including ones that require other services";
          category = "[testing]";
        }
        {
          name = "clippy";
          command = "cargo clippy --all-targets --no-deps";
          help = "Run clippy on all targets";
          category = "[cargo actions]";
        }
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
            docker run --rm --network host grid:latest
          '';
          help = "Runs the site binary in a container.";
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

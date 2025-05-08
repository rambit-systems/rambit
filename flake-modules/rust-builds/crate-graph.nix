{ ... }: {
  perSystem = { pkgs, rust-toolchain, rust-workspace, ... }: let
    inherit (rust-workspace.workspace-base-args) src;
    inherit (rust-toolchain) craneLib;

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
      nativeBuildInputs = [ pkgs.graphviz ];
    };
  in {
    packages = { inherit crate-graph crate-graph-image; };
  };
}

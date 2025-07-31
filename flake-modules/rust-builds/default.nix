{ ... }: {
  imports = [
    ./workspace.nix
    ./crate-graph.nix
    ./cli.nix
    ./site.nix
  ];
}

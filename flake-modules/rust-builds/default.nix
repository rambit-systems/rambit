{ ... }: {
  imports = [
    ./workspace.nix
    ./crate-graph.nix
    ./app.nix
    ./cli.nix
  ];
}

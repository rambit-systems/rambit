{ withSystem, ... }: {
  flake.nixosModules = {
    tikv = (import ./tikv.nix) { inherit withSystem; };
    pd = (import ./pd.nix) { inherit withSystem; };
    api = (import ./api.nix) { inherit withSystem; };
  };
}

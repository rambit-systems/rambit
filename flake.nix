{
  inputs = {
    flake-parts.url = "https://flakehub.com/f/hercules-ci/flake-parts/0.1.tar.gz";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    nix-filter.url = "github:numtide/nix-filter";
    devshell.url = "github:numtide/devshell";

    rust-overlay = {
      url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "https://flakehub.com/f/ipetkov/crane/0.20.tar.gz";
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } ({ ... }: {
    systems = [ "x86_64-linux" "aarch64-linux" ];

    imports = [
      ./flake-modules/nixpkgs
      ./flake-modules/rust-toolchain
      ./flake-modules/rust-builds
      ./flake-modules/repo-tests
      ./flake-modules/e2e-tests
      ./flake-modules/devshell
    ];
  });
}

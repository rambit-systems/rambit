{ ... } @ moduleTop: {
  perSystem = { pkgs, ... } @ perSystemTop: let
    # prefix for test checks
    test-prefix = "nixvm-test-";

    # the common module
    common = import ./common.nix (moduleTop // perSystemTop);
    # the args we'll give to each module
    module-args = moduleTop // perSystemTop // { inherit common; };
    # helper to call a module with the args
    call = source: import source module-args;
    # renames attrset keys and adds test check prefix
    test-renamer = name: value: pkgs.lib.attrsets.nameValuePair "${test-prefix}${name}" value;
    # helper to call a test module
    callTestModule = source: pkgs.lib.attrsets.mapAttrs' test-renamer (call source);
  in {
    checks = { }
      # // (callTestModule ./domain/upload-download.nix)
      # // (callTestModule ./domain/authenticate-deauthenticate.nix)
    ;
  };
}

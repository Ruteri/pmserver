{ nixpkgs ? <nixpkgs>
, system ? builtins.currentSystem
}:

let
  pkgs = import nixpkgs { inherit system; };

  callPackage = pkgs.lib.callPackageWith (pkgs // pkgs.xlibs // self);

  self = rec {
    pmserver = callPackage ./derivation.nix { };
  };

  entrypoint = pkgs.writeScript "entrypoint.sh" ''
    #!${pkgs.stdenv.shell}
    set -e
    pmserver
  '';
in
pkgs.dockerTools.buildImage {
  name = "pmserver-docker";
  runAsRoot = ''
    #!${pkgs.stdenv.shell}
    ${pkgs.dockerTools.shadowSetup}
  '';

  contents = [ self.pmserver ];

  config = {
    Cmd = [ "pmserver" ];
    Entrypoint = [ entrypoint ];
    ExposedPorts = {
      "8080/tcp" = {};
    };
    WorkingDir = "/app";
    Volumes = {
      "/app/data" = {};
    };
  };
}

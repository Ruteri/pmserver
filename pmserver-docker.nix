{ nixpkgs ? <nixpkgs>
, system ? builtins.currentSystem
, pmserver
}:

let
  pkgs = import nixpkgs { inherit system; };

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

  contents = [ pmserver ];

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

{ stdenv, rustPlatform }:

rustPlatform.buildRustPackage rec {
  name = "pmserver-${version}";
  version = "0.1.0";
  src = ./.;
  buildInputs = [];

  checkPhase = "";
  cargoSha256 = "sha256:1vg0kfj2kiv5qiil2x26x5i1z52ca54v2hgwbr5j0kymcn1lhyx8";

  meta = with stdenv.lib; {
    description = "Minimal user configuration storage for hdpwm";
    homepage = https://github.com/ruteri/pmserver;
    license = licenses.isc;
    platforms = platforms.all;
  };
}

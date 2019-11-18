# pmserver
Basic user state server for [hdpwm](https://github.com/Ruteri/hdpwm)


### Compilation

Rust binary: `cargo build`.


Nix package: `nix-build`.


Install nix package: `nix-env -f default.nix -i pmserver`.


NixOS-based docker builder (not for use as runtime container) `docker build -t pmserver:<version> .`.


### Running

`pmserver` binary is configured via the following env variables:
* `SESSION_KEY_SEED` (required) - string used for encrypting sessions (salted sha256 is used as aes256 key),
* `DB_PATH` (optional, defaults to `./data/db`) - leveldb path.

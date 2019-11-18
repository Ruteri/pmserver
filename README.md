# pmserver
Basic user state server for [hdpwm](https://github.com/Ruteri/hdpwm)


### Compilation

Rust binary: `cargo build`.


Nix package: `nix-build`.


NixOS-based docker image `docker build -t pmserver:<version> .`.


### Running

`pmserver` binary is configured via the following env variables:
* `SESSION_KEY_SEED` (required) - string used for encrypting sessions (salted sha256 is used as aes256 key),
* `DB_PATH` (optional, defaults to `./data/db`) - leveldb path.

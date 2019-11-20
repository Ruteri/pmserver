# pmserver
Basic user state server for [hdpwm](https://github.com/Ruteri/hdpwm)


### Compilation

Rust binary: `cargo build`.


Nix package: `nix-build`.


Install nix package:
 * from source: `nix-env -f default.nix -i pmserver`,
 * from cachix (from within a cloned repo): ```
nix-env -iA cachix -f https://cachix.org/api/v1/install
cachix use ruteri
nix-env -bif default.nix pmserver
```


Docker minimal (~30MB) runtime container: `nix-build docker.nix`.


NixOS-based docker builder (not for use as runtime container) `docker build -t pmserver .`.


### Running

`pmserver` binary is configured via the following env variables:
* `SESSION_KEY_SEED` (required) - string used for encrypting sessions (salted sha256 is used as aes256 key),
* `DB_PATH` (optional, defaults to `./data/db`) - leveldb path.

# pmserver
Basic user state server for [hdpwm](https://github.com/Ruteri/hdpwm)


### Compilation

* rust binary: `cargo build`,
* nix package: `nix-build -A pmserver`,
* minimal docker (~30MB) runtime container: `nix-build -A pmserver-docker && docker load < result`.
* NixOS-based docker builder `docker build -t pmserver .`.


### Installation

The following assumes a cloned repo: `git clone https://github.com/ruteri/pmserver && cd pmserver`.

* from source:
```
nix-env -f default.nix -iA pmserver
```

* from cachix:
```
nix-env -iA cachix -f https://cachix.org/api/v1/install
cachix use ruteri
nix-env -bif default.nix -A pmserver
```


### Running

`pmserver` binary is configured via the following env variables:
* `SESSION_KEY_SEED` (required) - string used for encrypting sessions (salted sha256 is used as aes256 key),
* `DB_PATH` (optional, defaults to `./data/db`) - leveldb path.

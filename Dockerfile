FROM nixos/nix

WORKDIR /app

COPY Cargo.lock .
COPY Cargo.toml .
COPY derivation.nix .
COPY default.nix .
COPY overlay.nix .

# RUN mkdir src && echo 'fn main() {}' > src/main.rs
# RUN nix-build (doesn't use cache)

COPY src src

RUN nix-build
RUN nix-store --gc
RUN nix-store --optimize

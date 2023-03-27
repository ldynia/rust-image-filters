# Install

Ubuntu installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://wasmtime.dev/install.sh -sSf | bash

rustup target add wasm32-wasi
rustup target add wasm32-unknown-unknown

rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
```

# Rust

## Cargo

Install libraries

```bash
cargo add image
cargo install wasm-pack

cargo install
```

Compile program

```bash
# debug
cargo build
./target/debug/instagrey assets/img/rainbow.jpg

# debian
cargo build --release --target x86_64-unknown-linux-gnu
./target/x86_64-unknown-linux-gnu/release/instagrey assets/img/rainbow.jpg

# alpine
cargo build --release --target x86_64-unknown-linux-musl
./target/x86_64-unknown-linux-musl/release/instagrey assets/img/rainbow.jpg

# wasm
cargo build --release --target wasm32-unknown-unknown
./target/release/instagrey assets/img/rainbow.jpg
```

## Manual compilation

```bash
rustc src/main.rs
./main assets/img/rainbow.ipg
```

## Docker

```bash
docker build --tag app --file devops/docker/Dockerfile .
docker run \
  --env OUTPUT_DIR=/tmp/instagrey \
  --volume $PWD/assets/img:/mnt/data \
  --volume $PWD/tmp:/tmp/instagrey \
  app instagrey /mnt/data/rainbow.jpg
```

# TODO

- https://donatstudios.com/Read-User-Files-With-Go-WASM
- https://stackoverflow.com/questions/51047146/how-to-read-a-file-with-javascript-to-webassembly

# Requirements

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://wasmtime.dev/install.sh -sSf | bash

rustup target add wasm32-wasi
rustup target add wasm32-unknown-unknown
rustup target add x86_64-unknown-linux-musl
```

## Rust

### Cargo

Install libraries

```bash
cargo add image
cargo install wasm-pack

cargo install
```

Compile program

```bash
cargo build
./target/debug/instagrey assets/img/rainbow.jpg

cargo build --release
./target/release/instagrey assets/img/rainbow.jpg
```

Run program

```bash
cargo instagrey assets/img/rainbow.jpg
```

## Manual compilation

```bash
rustc src/main.rs
./main assets/img/rainbow.ipg
```
 TODO
 
 https://donatstudios.com/Read-User-Files-With-Go-WASM

## Docker

```bash
docker build --tag app --file devops/docker/Dockerfile .
docker run \
  --env OUTPUT_DIR=/tmp/instagrey \
  --volume $PWD/assets/img:/mnt/data \
  --volume $PWD/tmp:/tmp/instagrey \
  app instagrey /mnt/data/rainbow.jpg
```

# Install

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

## Docker

```bash
docker build --tag app --file devops/docker/Dockerfile .
docker run \
  --env OUTPUT_DIR=/tmp/instagrey \
  --volume $PWD/assets/img:/mnt/data \
  --volume $PWD/tmp:/tmp/instagrey \
  app instagrey /mnt/data/rainbow.jpg
```

# Install

## Cargo

Install libraries

```bash
cargo add image

cargo install
```

Compile program

```bash
cargo build
./target/debug/run assets/img/rainbow.jpg

cargo build --release
./target/release/run assets/img/rainbow.jpg
```

Run program

```bash
cargo run assets/img/rainbow.jpg
```

## Manual compilation

```bash
rustc src/main.rs
./main assets/img/rainbow.ipg
```
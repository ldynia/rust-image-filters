# Install

## Cargo

Install libraries

```bash
cargo add time
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

# TODO

- https://stackoverflow.com/questions/57073697/how-to-convert-a-picture-in-pure-black-and-white-in-rust
# bevy_chess

Follow along of the bevy chess tutorial.

## Install

```bash
sudo apt-get install libasound2-dev libudev-dev libxcursor-dev \
  libxrandr-dev, libxi-dev, libx11-xcb-dev, libwayland-dev, xkbcommon
```

## Compile

```bash
cargo install -f cargo-binutils 
rustup component add llvm-tools-preview
cargo run --feature "bevy/dynamic_linking"
```
## Release

Support Rust platforms: https://doc.rust-lang.org/nightly/rustc/platform-support.html

### Windows

```bash
rustup target add rustup target add x86_64-pc-windows-msvc
cargo build --release --example spotlight --target x86_64-pc-windows-gnu
```

### Linux

```bash
rustup target add rustup target add x86_64-unknown-linux-gnu
cargo build --release --example spotlight --target x86_64-unknown-linux-gnu
```

### Web

1. Setup

  ```bash
  rustup target add wasm32-unknown-unknown
  cargo install wasm-bindgen-cli
  cargo install basic-http-server
  ```

#### WASM with WebGL2

1. Build

  ```bash
  # create binary
  cargo build --release --example spotlight --target wasm32-unknown-unknown --features webgl2

  # create javascript bindings
  wasm-bindgen --out-name spotlight \
    --out-dir examples/wasm/spotlight/target \
    --target web target/wasm32-unknown-unknown/release/examples/spotlight.wasm

1. Run

  ```bash
  basic-http-server examples/wasm/spotlight
  ```

#### WASM with WebGPU


```bash
cargo run -p build-wasm-example -- --api webgpu spotlight
```
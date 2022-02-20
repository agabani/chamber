# Contributing

## Setting up your local development environment

### Add the Wasm target

```bash
rustup target add wasm32-unknown-unknown
```

### Install [cargo watch](https://github.com/watchexec/cargo-watch)

```bash
cargo install cargo-watch
```

### Install [Trunk](https://github.com/thedodd/trunk)

```bash
cargo install --locked trunk
```

### Install [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)

```bash
cargo install --locked wasm-bindgen-cli
```

## Running your local development environment

### Run [Chamber API][1]

```bash
cargo watch -x 'run --bin chamber-api'
```

### Run [Chamber App][2]

```bash
trunk watch --dist ./dist -- packages/chamber-app/index.html
```

## Building for production environment

### Build [Chamber API][1]

```shell
cargo build --release --bin chamber-api
```

### Build [Chamber App][2]

```bash
trunk build --dist ./dist --release -- packages/chamber-app/index.html
```


[1]: </packages/chamber-api/>
[2]: </packages/chamber-app/>

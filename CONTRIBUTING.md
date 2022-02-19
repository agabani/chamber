# Contributing

## Setting up your local development environment

### Add the Wasm target

```bash
rustup target add wasm32-unknown-unknown
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

### Run [Chamber App][1]

```bash
trunk serve -- packages/chamber-app/index.html
```

## Building for production environment

### Build [Chamber App][1]

```bash
trunk build --release -- packages/chamber-app/index.html
```


[1]: </packages/chamber-app/>

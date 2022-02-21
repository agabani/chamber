# 1: Build
FROM rust:1.58.1 as builder

# 1a: Prepare toolchain
RUN apt update && \
    apt install -y musl-tools musl-dev && \
    rustup target add wasm32-unknown-unknown && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo install --locked trunk && \
    cargo install --locked wasm-bindgen-cli

# 1b: Download and compile Rust dependencies using fake source code and store as a separate Docker layer
WORKDIR /home/appuser/app

COPY packages/chamber/Cargo.toml packages/chamber/Cargo.toml
COPY .docker/lib.rs packages/chamber/src/lib.rs

COPY packages/chamber-api/Cargo.toml packages/chamber-api/Cargo.toml
COPY .docker/main.rs packages/chamber-api/src/main.rs

COPY packages/chamber-app/Cargo.toml packages/chamber-app/Cargo.toml
COPY packages/chamber-app/index.html packages/chamber-app/index.html
COPY .docker/index.scss packages/chamber-app/scss/index.scss
COPY .docker/main.rs packages/chamber-app/src/main.rs

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN cargo build --target x86_64-unknown-linux-musl --release && \
    trunk build --dist ./dist --release -- packages/chamber-app/index.html && \
    rm -rf dist/ packages/ target/wasm-bindgen/ && \
    find ./target/ -regex ".*chamber-app.*" -delete

# 1c: Build the application using the real source code
COPY packages/ packages/

RUN cargo build --target x86_64-unknown-linux-musl --release && \
    trunk build --dist ./dist --release -- packages/chamber-app/index.html

# 2: Copy the excutable and extra files to an empty Docker image
FROM scratch

COPY --chown=root:root .docker/passwd /etc/passwd
COPY --chown=root:root .docker/group /etc/group

USER appuser:appgroup

ENV CHAMBER_API__HTTP_SERVER__HOST=0.0.0.0
ENV CHAMBER_API__HTTP_SERVER__PORT=80
ENV CHAMBER_API__STATIC_FILES__DIRECTORY=dist/

EXPOSE 80

WORKDIR /home/appuser/app

COPY --chown=appuser:appgroup --from=builder /home/appuser/app/target/x86_64-unknown-linux-musl/release/chamber-api chamber-api
COPY --chown=appuser:appgroup --from=builder /home/appuser/app/dist/ dist/

CMD [ "./chamber-api" ]

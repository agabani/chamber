set shell := ["bash", "-uc"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# help
help:
  @just --list

# format
format:
  @cargo fmt

# lint
lint:
  @cargo clippy

# run
run:
  @cargo run

# test
test:
  @cargo test

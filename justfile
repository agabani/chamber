set shell := ["bash", "-uc"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# help
help:
  @just --list

# container registry run
container-registry-run:
  @docker compose up -d
  @docker pull ubuntu
  @docker image tag ubuntu localhost:5000/ubuntu
  @docker push localhost:5000/ubuntu

# container registry stop
container-registry-stop:
  @docker compose down

# format
format:
  @cargo fmt

# lint
lint:
  @cargo clippy

# mkdocs build
mkdocs-build:
  @mkdocs build

# mkdocs install
mkdocs-install:
  @pip3 install --upgrade --user mkdocs
  @pip3 install --upgrade --user mkdocs-include-markdown-plugin

# mkdocs serve
mkdocs-serve:
  @mkdocs serve

# run
run:
  @cargo run

# test
test:
  @cargo test

set shell := ["bash", "-uc"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# help
help:
  @just --list

# container registry run
container-registry-run:
  @docker compose up --detach

# container registry seed copy
container-registry-seed-copy:
  @docker run --rm --net host -v ./skopeo:/tmp quay.io/skopeo/stable:latest copy docker://ubuntu:latest docker://localhost:5000/ubuntu:oci --dest-tls-verify=false --format=oci --multi-arch all
  @docker run --rm --net host -v ./skopeo:/tmp quay.io/skopeo/stable:latest copy docker://ubuntu:latest docker://localhost:5000/ubuntu:v2 --dest-tls-verify=false --format=v2s2 --multi-arch all
  @docker run --rm --net host -v ./skopeo:/tmp quay.io/skopeo/stable:latest copy docker://ubuntu:latest docker://localhost:5001/ubuntu:oci --dest-tls-verify=false --format=oci --multi-arch all --dest-creds admin:password
  @docker run --rm --net host -v ./skopeo:/tmp quay.io/skopeo/stable:latest copy docker://ubuntu:latest docker://localhost:5001/ubuntu:v2 --dest-tls-verify=false --format=v2s2 --multi-arch all --dest-creds admin:password
  @docker run --rm --net host -v ./skopeo:/tmp quay.io/skopeo/stable:latest copy docker://ubuntu:latest docker://localhost:5002/ubuntu:oci --dest-tls-verify=false --format=oci --multi-arch all --dest-creds admin:password
  @docker run --rm --net host -v ./skopeo:/tmp quay.io/skopeo/stable:latest copy docker://ubuntu:latest docker://localhost:5002/ubuntu:v2 --dest-tls-verify=false --format=v2s2 --multi-arch all --dest-creds admin:password

# container registry seed login
container-registry-seed-login:
  @docker run --rm --net host -v ./skopeo:/tmp -it quay.io/skopeo/stable:latest login docker.io

# container registry stop
container-registry-stop:
  @docker compose down --remove-orphans --volumes

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

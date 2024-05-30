FROM mcr.microsoft.com/devcontainers/rust:1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
     && apt-get -y install --no-install-recommends postgresql-client \
     && cargo install sea-orm-cli cargo-insta \
     && chown -R vscode /usr/local/cargo \
     && rustup toolchain install nightly \
     && rustup default nightly \
     && rustup target add wasm32-unknown-unknown \
     && cargo install cargo-leptos \
     && cargo install trunk

COPY .env /.env

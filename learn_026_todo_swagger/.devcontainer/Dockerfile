# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/devcontainers/rust:1-${VARIANT}

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -y && apt-get clean -y \
    && apt-get -y install libpq-dev python-dev \
    && apt install python3-pip -y \
    && pip install pgcli \
    && cargo install cargo-watch \
    && apt install nodejs \
    && apt install npm \
    && npm install -g yarn \
    && npm i @vue/cli-service \
    && npm install -g @vue/cli \
    # && npm install @babel/core @babel/preset-env

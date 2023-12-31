FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -y && apt-get clean -y \
    && apt install python3 
    #&& apt install python3-virtualenv
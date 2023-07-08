# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

RUN cargo install cargo-watch
# cargo runが上のせいで動かなくなってた。
# /usr/local/cargo/registryでPermisson Dinedが起きていた
# rustlangグループにvscodeグループとして参加してる
RUN chmod g+rw -R /usr/local/cargo/registry/ 
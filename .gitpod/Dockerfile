FROM gitpod/workspace-full:latest

RUN sudo apt-get install -yq \
        musl \
        musl-dev \
        musl-tools \
        libssl-dev \
    && sudo apt-get clean && sudo rm -rf /var/lib/apt/lists/* /tmp/*

RUN bash -c "CARGO_HOME=/home/gitpod/.cargo rustup target add x86_64-unknown-linux-musl"

RUN bash -c "CARGO_HOME=/home/gitpod/.cargo cargo install cargo-watch"
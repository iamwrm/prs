FROM rust:1.72

RUN \
    cargo install cargo-zigbuild

# install zig
RUN \
    curl -fsSL https://bun.sh/install | bash && \
    bash -c 'source /root/.bashrc && bun install -y zig && which zig && zig version'

    


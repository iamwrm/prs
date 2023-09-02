FROM rust:1.72

RUN \
    cargo install cargo-zigbuild

# install zig
RUN \
    curl -fsSL https://bun.sh/install | bash && \
    bun install -y zig

    


FROM docker.io/rust:1

RUN \
    cargo install cargo-zigbuild

# install zig
RUN \
    curl -fsSL https://bun.sh/install | bash && \
    bash -c 'source /root/.bashrc && bun install -g @oven/zig && which zig && zig version'

    


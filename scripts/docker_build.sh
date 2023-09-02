#!/bin/bash

set -ueo pipefail

export DOCKER_BUILDKIT=1

export DOCKER=podman


${DOCKER} build \
	-t rust_builder \
	-f docker/Dockerfile.builder . 

${DOCKER} run --rm \
	-v ${PWD}:/app \
	rust_builder \
	cargo zigbuild --target x86_64-unknown-linux-gnu.2.17

ls -lah ./target/x86_64-unknown-linux-gnu.2.17/release/prs

${DOCKER} build -t prs \
	-f docker/Dockerfile.runtime .

sudo bash -c "find . -type d -print0 | xargs -0 chmod 0755" &
sudo bash -c "find . -type f -print0 | xargs -0 chmod 0644" &

wait

# extract the binary from the docker image
${DOCKER} run --rm -v ${PWD}:/host prs bash -c "cp /usr/local/bin/prs /host && chown 1000:1000 /host/prs"

ls -lah ./prs

echo "Checking GLIBC requirements"
objdump -T ./prs | grep GLIBC | sed 's/.*GLIBC_\([.0-9]*\).*/\1/g' | sort -Vu

${DOCKER} run --rm -it prs prs -p top10-mem

sudo chmod +x ./prs
./prs -p top10-mem

wget -q https://github.com/upx/upx/releases/download/v4.0.2/upx-4.0.2-amd64_linux.tar.xz
tar -xf upx-4.0.2-amd64_linux.tar.xz
ls -lah ./prs
./upx-4.0.2-amd64_linux/upx --ultra-brute ./prs
ls -lah ./prs
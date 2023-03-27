#!/bin/bash

set -ueo pipefail

export DOCKER_BUILDKIT=1

export DOCKER=podman


${DOCKER} build -t rust_centos7_builder \
	-f docker/Dockerfile.builder . 

${DOCKER} build -t rust_ubi9_builder \
	-f docker/Dockerfile.ubi9 . 

${DOCKER} run --rm \
	-v ${PWD}:/app \
	-v ${HOME}/.cargo/registry/cache:/home/u1000/.cargo/registry/cache \
	-v ${HOME}/.cargo/registry/index:/home/u1000/.cargo/registry/index \
	rust_centos7_builder \
		bash -c "sudo chmod -R 777 ~/.cargo \
            && sudo chmod -R 777 /app \
            && source ~/.cargo/env \
            && cd /app \
            && cargo clean \
            && cargo build --release"

${DOCKER} build -t prs \
	-f docker/Dockerfile.runtime .

sudo bash -c "find . -type d -print0 | xargs -0 chmod 0755" &
sudo bash -c "find . -type f -print0 | xargs -0 chmod 0644" &

wait

# extract the binary from the docker image
${DOCKER} run --rm -v ${PWD}:/host prs bash -c "cp /usr/local/bin/prs /host && chown 1000:1000 /host/prs"

ls -lah ./prs
objdump -T ./prs | grep GLIBC | sed 's/.*GLIBC_\([.0-9]*\).*/\1/g' | sort -Vu

${DOCKER} run --rm -it prs prs -p top10-mem

sudo chmod +x ./prs
./prs -p top10-mem

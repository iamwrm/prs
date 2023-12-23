#!/bin/bash

set -ueo pipefail

export DOCKER_BUILDKIT=1
export DOCKER=docker


${DOCKER} build \
	-t rust_builder \
	-f docker/Dockerfile.builder . 

${DOCKER} run --rm \
	--init \
	-v ${PWD}:/app \
	rust_builder \
	bash -c "source /root/.bashrc && \
		cd /app && \
		cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17"

ls -lah ./target/x86_64-unknown-linux-gnu/release/prs

${DOCKER} build -t prs \
	-f docker/Dockerfile.runtime .

sudo bash -c "find . -type d -print0 | xargs -0 chmod 0755" &
sudo bash -c "find . -type f -print0 | xargs -0 chmod 0644" &

wait

# extract the binary from the docker image
${DOCKER} run \
	--rm \
	-v ${PWD}:/host \
	prs bash -c "cp /usr/local/bin/prs /host && chown 1000:1000 /host/prs"

ls -lah ./prs
sudo chmod +x ./prs

echo "Checking GLIBC requirements"
objdump -T ./prs | grep GLIBC | sed 's/.*GLIBC_\([.0-9]*\).*/\1/g' | sort -Vu


${DOCKER} run --rm prs bash -c "whoami && chmod +x /usr/local/bin/prs && /usr/local/bin/prs -p top10-mem"

./prs -p top10-mem

bash scripts/release/setup_upx.sh

export PATH=${PWD}/local_data/bin:$PATH

ls -lah ./prs
upx --ultra-brute ./prs
ls -lah ./prs
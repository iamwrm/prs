#!/bin/bash

set -ueo pipefail

export DOCKER_BUILDKIT=1

export DOCKER=podman

# load 2 docker images if they exist

if [ -f prs.tar ]; then
    echo "Loading prs.tar"
    ${DOCKER} load -i prs.tar
fi

if [ -f rust_centos7_builder.tar ]; then
    echo "Loading rust_centos7_builder.tar"
    ${DOCKER} load -i rust_centos7_builder.tar
fi

${DOCKER} build -t rust_centos7_builder \
	-f docker/Dockerfile.builder . 

${DOCKER} run --rm \
	-v ${PWD}:/app \
	-v ${HOME}/.cargo/registry/cache:/home/u1000/.cargo/registry/cache \
	-v ${HOME}/.cargo/registry/index:/home/u1000/.cargo/registry/index \
	rust_centos7_builder \
		bash -c "sudo chmod -R 777 ~/.cargo \
            && sudo chmod -R 777 /app \
            && source ~/.cargo/env \
            && cd /app \
            && cargo build --release"

${DOCKER} build -t prs \
	-f docker/Dockerfile.runtime .

find . -type d -print0 | xargs -0 chmod 0755 &
find . -type f -print0 | xargs -0 chmod 0644 &

wait
# export 2 docker images 

rm -f prs.tar rust_centos7_builder.tar

${DOCKER} save -o prs.tar prs &
${DOCKER} save -o rust_centos7_builder.tar rust_centos7_builder &

wait

ls -alh *.tar
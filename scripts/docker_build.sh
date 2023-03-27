#!/bin/bash

set -ueo pipefail

export DOCKER_BUILDKIT=1

# load 2 docker images if they exist

if [ -f prs.tar ]; then
    echo "Loading prs.tar"
    docker load -i prs.tar
fi

if [ -f rust_centos7_builder.tar ]; then
    echo "Loading rust_centos7_builder.tar"
    docker load -i rust_centos7_builder.tar
fi

docker build -t rust_centos7_builder \
	-f docker/Dockerfile.builder . 

docker run --rm \
	-v ${PWD}:/app \
	-v ${HOME}/.cargo/registry/cache:/home/u1000/.cargo/registry/cache \
	-v ${HOME}/.cargo/registry/index:/home/u1000/.cargo/registry/index \
	-v ${HOME}/.cargo/git/db:/home/u1000/.cargo/git/db \
	rust_centos7_builder \
		bash -c "sudo chmod -R 777 ~/.cargo && ls -lah ~/.cargo/registry && source ~/.cargo/env && cd /app && cargo build --release"

docker build -t prs \
	-f docker/Dockerfile.runtime .

# export 2 docker images 

docker save -o prs.tar prs
docker save -o rust_centos7_builder.tar rust_centos7_builder


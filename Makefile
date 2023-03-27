.PHONY: $(MAKECMDGOALS)

export DOCKER_BUILDKIT=1

help:
	echo "help"

docker_build:
	docker build -t rust_centos7_builder \
		-f docker/Dockerfile.builder . 
	docker run --rm \
		-v ${PWD}:/app \
		-v ${HOME}/.cargo/registry/cache:/home/u1000/.cargo/registry/cache \
		-v ${HOME}/.cargo/registry/index:/home/u1000/.cargo/registry/index \
		-v ${HOME}/.cargo/git/db:/home/u1000/.cargo/git/db \
		rust_centos7_builder \
			bash -c "source ~/.cargo/env && cd /app && id && cargo build --release"
	docker build -t prs \
		-f docker/Dockerfile.runtime .
docker_run:
	docker run --rm -it prs prs -p top10-mem
docker_extract:
	docker run --rm -v ${PWD}:/host prs bash -c "cp /usr/local/bin/prs /host && chown 1000:1000 /host/prs"
run:
	cargo r -- -p top10-mem


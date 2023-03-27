.PHONY: $(MAKECMDGOALS)


help:
	echo "help"

docker_build:
	bash scripts/docker_build.sh
docker_run:
	docker run --rm -it prs prs -p top10-mem
docker_extract:
	docker run --rm -v ${PWD}:/host prs bash -c "cp /usr/local/bin/prs /host && chown 1000:1000 /host/prs"
run:
	cargo r -- -p top10-mem


.PHONY: $(MAKECMDGOALS)

help:
	echo "help"

docker_build:
	docker build -t prs . 
docker_run:
	docker run --rm -it prs prs -p top10-mem
run:
	cargo r -- -p top10-mem

install:
	./run.sh bun install

new_dep:
	./run.sh bun install $(name)

format:
	./run.sh bun format

lint:
	./run.sh bun lint
	make format

run:
	./run.sh bun dev --host 0.0.0.0

build:
	./run.sh bun run build

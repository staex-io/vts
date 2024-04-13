install:
	./run.sh bun install

new_dep:
	./run.sh bun install $(name)

format:
	./run.sh bun format

lint: format
	./run.sh bun lint

run:
	./run.sh bun dev --host 0.0.0.0

build:
	./run.sh bun run build

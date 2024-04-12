#!/bin/bash

OS=$(uname -s)
if [ "$OS" == "Linux" ]; then
    exec docker run --rm \
        --user "$(id -u):$(id -g)" \
        -it \
        -v "${PWD}":/rise \
        --entrypoint="" \
        --workdir /rise \
        --network=host \
        oven/bun "$@"
elif [ "$OS" == "Darwin" ]; then
    exec docker run --rm \
        --user "$(id -u):$(id -g)" \
        -p 5173:5173 \
        -it \
        -v "${PWD}":/rise \
        --entrypoint="" \
        --workdir /rise \
        oven/bun "$@"
else
    echo "Your operating system is not recognized as Linux or macOS."
fi

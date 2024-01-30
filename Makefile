.PHONY: all build serve dependencies package

all: build serve

.SHELL := /bin/bash

dependencies:
	./install-deps.sh

dev-dependencies:
	DEV=1 ./install-deps.sh

build: dependencies
	wasm-pack build --target web

serve:
	python3 -m http.server 8080

watch: dev-dependencies
	bacon

package: build
	rm -rf build
	mkdir build
	cp -r pkg build
	cp index.html build/
.PHONY: build test format clean

all: build test

build:
	mix deps.get
	mix compile

test:
	mix test

format:
	mix format "{lib,test}/**/*.{ex,exs}"

clean:
	mix clean
	-rm -rf _build
	-rm -rf deps

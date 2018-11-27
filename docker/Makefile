.PHONY: test

APP_DIR = $(shell cd ../ && pwd)

all: test

test:
	docker run -it --rm -v $(APP_DIR):/app alinpopa/rustler:rust1.30.1-elixir1.7.4 sh -c "cd /app/ ; make"

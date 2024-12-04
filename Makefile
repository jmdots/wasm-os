.PHONY: all build serve clean update setup

all: build serve

# Build the WebAssembly package
build:
	wasm-pack build --target web
	rm -rf www/pkg
	cp -r pkg www/

# Start the development server
serve:
	cd www && python server.py

# Clean build artifacts
clean:
	rm -rf pkg
	rm -rf www/pkg
	cargo clean

# Setup development environment
setup:
	rustup target add wasm32-unknown-unknown
	cargo install wasm-pack
	cargo install cargo-watch

# Build and watch for changes
watch:
	cargo watch -i pkg -i www/pkg -s "make build"

# Run development mode with auto-rebuild
dev: build
	make watch & make serve

# Update dependencies
update:
	cargo update
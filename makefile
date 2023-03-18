all: release

release: UI
	cargo build --release

UI: 
	cd frontend && npm run build

run: all
	cargo run --release

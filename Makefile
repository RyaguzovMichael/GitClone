all: build copy

build:
	cargo build  --release

copy:
	cp ./target/release/gclone $$HOME/bin/

all: release

init:
	cargo init
	cargo add regex

release:
	cargo build --release

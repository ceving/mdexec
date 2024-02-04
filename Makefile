all: release

init:
	cargo init
	cargo add regex

release:
	cargo build --release

example: release
	target/release/mdexec README.md bash

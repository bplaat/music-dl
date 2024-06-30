#!/bin/sh

set -e

if [ "$1" = "clean" ]; then
	cargo clean
fi

if [ "$1" = "check" ]; then
	./meta.sh check-scripts
	./meta.sh check-rust
fi

if [ "$1" = "check-scripts" ]; then
	shellcheck meta.sh
fi

if [ "$1" = "check-rust" ]; then
	cargo fmt -- --check
	cargo clippy -- -Dwarnings -W clippy::unwrap_used
	cargo machete
fi

if [ "$1" = "run" ]; then
	shift
	cargo run -- "$@"
fi

if [ "$1" = "release" ]; then
	cargo build --release
fi

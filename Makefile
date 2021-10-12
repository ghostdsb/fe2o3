push:
	cargo-clippy
	cargo test
	cargo fmt
	git add .
	git cz
	git push
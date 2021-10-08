push:
	cargo test
	cargo test -- --ignored
	cargo fmt
	git add .
	git cz
	git push
run:
	cargo run
watch:
	cargo watch -x run
build:
	cargo build --release --target x86_64-unknown-linux-musl
preview:
	./target/x86_64-unknown-linux-musl/release/examples
test:
	cargo test
deploy:
	docker build -t axum-app .
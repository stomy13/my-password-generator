run:
	cargo run
build:
	cargo build --release
install:
	sudo ln -s ~/dev/my-password-generator/target/release/genpass /usr/local/bin/genpass
build:
	cargo build --release

install:
#cargo install --path . --root /usr/local/bin
	cargo install --path .
	sudo ln -s ${HOME}/.cargo/bin/i4 /usr/local/bin/i4

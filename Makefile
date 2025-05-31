LLVM_PROFDATA := $(shell find ${HOME}/.rustup/toolchains -name llvm-profdata)
LLVM_COV := $(shell find ${HOME}/.rustup/toolchains -name llvm-cov)
OBJECT_NAME := $(shell find target/debug/deps -name "i4-[a-z0-9]*")

build:
	cargo build --release

install:
#cargo install --path . --root /usr/local/bin
	cargo install --path .
	sudo ln -s ${HOME}/.cargo/bin/i4 /usr/local/bin/i4

# https://doc.rust-lang.org/stable/rustc/instrument-coverage.html
coverage:
	echo "${LLVM_PROFDATA}"
	rm -f *.profraw *.profdata *.log
	RUSTFLAGS="-C instrument-coverage" cargo test --tests 2> tmp/test_output.log
#	RUSTFLAGS="-C instrument-coverage" cargo test --tests | tee test_output.log >&2
#	RUSTFLAGS="-C instrument-coverage" cargo test --tests | tee test_output.log 2>&1
#	OBJECT_NAME = $(shell grep -o "target/debug/deps/i4-[a-z0-9]*" test_output.log | head -n 1)
	${LLVM_PROFDATA} merge -sparse default_*.profraw -o i4.profdata
	${LLVM_COV} report --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=i4.profdata --object $(shell grep -o "target/debug/deps/i4-[a-z0-9]*" test_output.log | head -n 1)
#	${LLVM_COV} show --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=i4.profdata --object $(shell grep -o "target/debug/deps/i4-[a-z0-9]*" test_output.log | head -n 1) --show-instantiations --show-line-counts-or-regions --line-coverage-lt=1 --Xdemangler=rustfilt | less -R

.PHONY: test coverage clippy clean

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

CAIRO0_PROGRAMS_DIR=cairo_programs/cairo0
CAIRO0_PROGRAMS:=$(wildcard $(CAIRO0_PROGRAMS_DIR)/*.cairo)
COMPILED_CAIRO0_PROGRAMS:=$(patsubst $(CAIRO0_PROGRAMS_DIR)/%.cairo, $(CAIRO0_PROGRAMS_DIR)/%.json, $(CAIRO0_PROGRAMS))

# Variable to give compiled Cairo programas a proper name. It extracts the file extension and adds
# the .json extension to it.
COMPILED_PROGRAM=$(basename $(PROGRAM)).json

# Rule to compile Cairo programs for testing purposes.
# If the `cairo-lang` toolchain is installed, programs will be compiled with it.
# Otherwise, the cairo_compile docker image will be used
# When using the docker version, be sure to build the image using `make docker_build_cairo_compiler`.
$(CAIRO0_PROGRAMS_DIR)/%.json: $(CAIRO0_PROGRAMS_DIR)/%.cairo
	@echo "Compiling Cairo program..."
	@cairo-compile --cairo_path="$(CAIRO0_PROGRAMS_DIR)" $< --output $@ 2> /dev/null --proof_mode || \
	docker run --rm -v $(ROOT_DIR)/$(CAIRO0_PROGRAMS_DIR):/pwd/$(CAIRO0_PROGRAMS_DIR) cairo --proof_mode /pwd/$< > $@

build: 
	cargo build --release

test: $(COMPILED_CAIRO0_PROGRAMS)
	cargo test

test_metal: $(COMPILED_CAIRO0_PROGRAMS)
	cargo test -F metal

docker_build_cairo_compiler:
	docker build -f cairo_compile.Dockerfile -t cairo .

docker_compile_cairo:
	docker run -v $(ROOT_DIR):/pwd cairo --proof_mode /pwd/$(PROGRAM) > $(COMPILED_PROGRAM)

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

benchmarks_sequential: $(COMPILED_CAIRO0_PROGRAMS)
	cargo bench

benchmarks_parallel: $(COMPILED_CAIRO0_PROGRAMS)
	cargo bench -F parallel --bench criterion_prover
	cargo bench -F parallel --bench criterion_verifier

benchmarks_parallel_all: $(COMPILED_CAIRO0_PROGRAMS)
	cargo bench -F parallel

build_metal:
	cargo b --features metal --release
	
clean:
	rm -f $(CAIRO0_PROGRAMS_DIR)/*.json
	rm -f $(CAIRO0_PROGRAMS_DIR)/*.trace
	rm -f $(CAIRO0_PROGRAMS_DIR)/*.memory

build_wasm:
	wasm-pack build --target=web -- --features wasm

test_wasm:
	wasm-pack test --node --release -- --features wasm 

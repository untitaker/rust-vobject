THIS_MAKEFILE_PATH:=$(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))
THIS_DIR:=$(shell cd $(dir $(THIS_MAKEFILE_PATH));pwd)

test:
	cargo test

build:
	cargo build

docs:
	cd "$(THIS_DIR)"
	cp src/vobject/lib.rs code.bak
	cat README.md | sed -e 's/^/\/\/! /g' > readme.bak
	sed -i '/\/\/ DOCS/r readme.bak' src/vobject/lib.rs
	cargo doc
	make clean

clean:
	cd "$(THIS_DIR)"
	mv code.bak src/vobject/lib.rs || true
	rm *.bak || true

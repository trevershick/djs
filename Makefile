TEST_EXE=$(shell ls -tcS target/debug/djs-* 2>/dev/null | head -n1)

KCOV=../kcov/build/src/Release/kcov
VPATH = src:../headers
TARGET = target
SRC=$(shell find src -name "*.rs")
DEBUG_EXE=target/debug/djs
SUBDIRS=doc

export DEBUG_EXE

.PHONY: default clean test coverage doc $(SUBDIRS)

default: test

all: test doc


clean:
	rm -rf $(TARGET)

$(DEBUG_EXE): $(SRC)
	cargo build

$(TEST_EXE): $(SRC)
	cargo test --no-run

test: $(TEST_EXE)
	cargo test

coverage: $(TEST_EXE)
	mkdir -p target/debug/coverage
	$(KCOV) target/debug/coverage $(TEST_EXE)

doc: $(DEBUG_EXE)
	$(MAKE) -C $@


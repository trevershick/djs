TEST_EXE=$(shell ls -tcS target/debug/djs-* 2>/dev/null | head -n1)

RBENV=$(shell which rbenv)
CHANGELOG_GENERATOR = $(shell which github_changelog_generator)
KCOV=../kcov/build/src/Release/kcov

DEBUG_EXE=target/debug/djs

SRC=$(shell find src -name "*.rs")
TARGET = target
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

doc: $(DEBUG_EXE) CHANGELOG.md
	$(MAKE) -C $@

changelog: $(CHANGELOG.md)

CHANGELOG.md: $(SRC)
	github_changelog_generator

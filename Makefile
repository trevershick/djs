TEST_EXE=$(shell ls -tcS target/debug/djs-* 2>/dev/null | head -n1)

PROJECT_ROOT = $(shell pwd)
RBENV=$(shell which rbenv)
CHANGELOG_GENERATOR = docker run -it --rm -v "$(pwd)":/usr/local/src/your-app ferrarimarco/github-changelog-generator trevershick/djs
KCOV=../kcov/build/src/Release/kcov

DEBUG_EXE=target/debug/djs

SRC=$(shell find src -name "*.rs")
TARGET = target
SUBDIRS=doc

export DEBUG_EXE PROJECT_ROOT

.PHONY: default clean test coverage doc generators

default: test

all: test doc generators

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

generators:
	$(MAKE) -C $@

doc: $(DEBUG_EXE) CHANGELOG.md
	$(MAKE) -C $@

changelog: $(CHANGELOG.md)

CHANGELOG.md: $(SRC)
	$(CHANGELOG_GENERATOR)

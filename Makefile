TEST_EXE=$(shell find target/debug -name "djs-*" -perm +111 -type f -depth 1)

KCOV=../kcov/build/src/Release/kcov

test:
	cargo test --no-run
	mkdir -p target/debug/coverage
	$(KCOV) target/debug/coverage $(TEST_EXE)

RELEASE_SHA=$(shell git rev-list --tags --max-count=1)
RELEASE_VERSION=$(shell git describe --tags $(RELEASE_SHA))

export RELEASE_VERSION

TARGET = target
MD = mkdir

target_dir:
	$(MD) -p $(TARGET)

README.html: README.md target_dir
	pandoc README.md --html > target/README.html

doc: README.html

homebrew:
	echo "Main Makefile, v=$(RELEASE_VERSION)"
	$(MAKE) -C pkg/brew


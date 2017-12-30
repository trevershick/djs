
PANDOC ?= pandoc
MANSECTION ?= 1
MANPAGE.md = $(PANDOC) --standalone $(PANDOCFLAGS) --to man
MANPAGE.gfm = $(PANDOC) --standalone $(PANDOCFLAGS) --to gfm
GITHUBREADME = ../README.md
GITHUBREADME_SRC = README.md.tmpl
GITHUBMANPAGE = ../MANPAGE.md
TMP_HELP_OPTS = help-options.tmp

MAN_PAGES=djs.1

README_SRCS=$(wildcard ??_README.pandoc.md)

.PHONY: clean all default
#.INTERMEDIATE: $(TMP_HELP_OPTS)

default: all

all: djs.1 $(GITHUBREADME) $(GITHUBMANPAGE)

clean:
	rm $(MAN_PAGES) $(TMP_HELP_OPTS)

$(TMP_HELP_OPTS): ../$(DEBUG_EXE)
	../$(DEBUG_EXE) -h > $(TMP_HELP_OPTS)

$(GITHUBREADME): $(TMP_HELP_OPTS) ../$(DEBUG_EXE)
	./preprocess.sh $(GITHUBREADME_SRC) > $@

$(GITHUBMANPAGE): djs.1
	$(MANPAGE.gfm) djs.1.md -o $(GITHUBMANPAGE)

%.$(MANSECTION): %.$(MANSECTION).md
	$(MANPAGE.md) $< -o $@

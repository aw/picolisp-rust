# Makefile to build the lib and test it

.PHONY: all

all: check

check:
		cargo build && \
		./extract.l

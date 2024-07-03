src/man.1: README.md
	scdoc < README.md > src/man.1

man: src/man.1
	man ./src/man.1

.PHONY: man

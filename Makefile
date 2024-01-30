help: # Display this help
	@cat Makefile | egrep '^[a-z0-9 ./-]*:.*#' | sed -E -e 's/:.+# */@ /g' -e 's/ .+@/@/g' | sort | awk -F@ '{printf "  \033[1;34m%-18s\033[0m %s\n", $$1, $$2}'
.PHONY: help

build: # Build the Rust crate and Python package
	maturin build
.PHONY: build

develop: # Install the crate as module in the current virtualenv, rehash pyenv to put CLI scripts in PATH
	maturin develop
	# Rehashing is needed (once) to make the [project.scripts] section of pyproject.toml available in the PATH
	pyenv rehash
.PHONY: develop

install: # Create the virtualenv
	pyenv virtualenv lab-python-rust
	pyenv local lab-python-rust
# Can't activate virtualenv from Makefile · Issue #372 · pyenv/pyenv-virtualenv
# https://github.com/pyenv/pyenv-virtualenv/issues/372
	@echo "Run 'pyenv activate' before running maturin commands"
.PHONY: install

uninstall: # Remove the virtualenv
	-rm .python-version
	-pyenv uninstall lab-python-rust
.PHONY: uninstall

example: develop # Run a simple example of Python code calling Rust code
	python -c "from lab_python_rust import _lab_python_rust as rust; print(rust.sum_as_string(5, 20)); print(rust.hello('John Doe'))"
.PHONY: example

cli: develop # Run the Python CLI script
	py-cli slug 'John Doe'
.PHONY: cli

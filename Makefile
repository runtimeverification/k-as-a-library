.PHONY: default all clean build install \
        poetry-install                  \
        test test-unit test-integration \
        format isort autoflake black    \
        check check-isort check-autoflake check-black check-flake8 check-mypy

POETRY_RUN := poetry run

default: check test-unit

all: check test

clean:
	rm -rf dist .mypy_cache
	find -type d -name __pycache__ -prune -exec rm -rf {} \;

distclean: clean
	$(POETRY_RUN) kbuild clean
	rm -rf .build

build:
	poetry build

poetry-install:
	poetry install


# Tests

test: test-unit test-integration

test-unit: poetry-install
	$(POETRY_RUN) pytest src/tests/unit --maxfail=1 --verbose $(TEST_ARGS)

test-integration: poetry-install
	$(POETRY_RUN) pytest src/tests/integration --numprocesses=4 --durations=0 --maxfail=1 --verbose $(TEST_ARGS)


# Checks and formatting

format: autoflake isort black
check: check-flake8 check-mypy check-autoflake check-isort check-black

check-flake8: poetry-install
	$(POETRY_RUN) flake8 src

check-mypy: poetry-install
	$(POETRY_RUN) mypy src

autoflake: poetry-install
	$(POETRY_RUN) autoflake --quiet --in-place src

check-autoflake: poetry-install
	$(POETRY_RUN) autoflake --quiet --check src

isort: poetry-install
	$(POETRY_RUN) isort src

check-isort: poetry-install
	$(POETRY_RUN) isort --check src

black: poetry-install
	$(POETRY_RUN) black src

check-black: poetry-install
	$(POETRY_RUN) black --check src

# Kompilation

kbuild-%:
	$(POETRY_RUN) kbuild kompile $*

install: kbuild-arithmetic
	mkdir -p .build/lib
	cp $(shell $(POETRY_RUN) kbuild which arithmetic)/lib* .build/lib

.PHONY: kbuild-*

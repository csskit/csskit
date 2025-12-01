# .mise-tasks

Task definitions for the mise task runner.

## Purpose

This directory contains task definitions used by the mise task runner for development workflows, build processes, and
automation scripts.

## Contents

- [`generate-values/`](./generate-values/) - Task for generating CSS value definitions.
- [`generate-csswg-changelog`](./generate-csswg-changelog) - Task for generating changelog from w3c/csswg-drafts commits.

## Tests

- `mise r csskit-acceptance` will run an acceptance test suite, executing commands using the `csskit` binary.

- `mise r generate-values <spec>` will generate Value definitions into `../crates/css_ast/src/values/<spec>/mod.rs`.

- `mise r generate-all-values` will generate Value definitions for all known CSS specs.

- `mise r generate-csswg-changelog [output-file]` will generate a changelog comparing the previous csswg-drafts SHA (stored in `.csswg-drafts-sha`) with the current main branch. Output defaults to `csswg-changelog.md`.

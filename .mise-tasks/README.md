# .mise-tasks

Task definitions for the mise task runner.

## Purpose

This directory contains task definitions used by the mise task runner for development workflows, build processes, and
automation scripts.

## Contents

- [`generate-values/`](./generate-values/) - Task for generating CSS value definitions.

## Tests

- `mise r csskit-acceptance` will run an acceptance test suite, executing commands using the `csskit` binary.

- `mise r generate-values <spec>` will generate Value definitions into `../crates/css_ast/src/values/<spec>/mod.rs`.

- `mise r generate-all-values` will generate Value definitions for all known CSS specs.

# Changelog

All notable changes to this project are documented in this file.

## [v1.1.1] - 2026-03-26

- **Feature:** Apply `cfg`/`cfg_attr` to match arms when provided in `attrs()` at field level ([ab4a13fb](https://github.com/vic1707/pud/commit/ab4a13fb)).
- **Feature:** Add struct-level `whole`/`whole = NAME` variant support ([fafd602e](https://github.com/vic1707/pud/commit/fafd602e)).

## [v1.1.0] - 2026-03-26

- **Feature:** Add field-level `#[pud(attrs(...))]` support ([3f5b48c](https://github.com/vic1707/pud/commit/3f5b48c)).
- **Feature:** Add struct-level `phantom_attrs(...)` and `group_attrs(NAME(...))` support ([8c17cc7](https://github.com/vic1707/pud/commit/8c17cc7)).
- **Chore:** Update deps ([3c89d71f](https://github.com/vic1707/pud/commit/3c89d71f)).

## [v1.0.0] - 2025-12-27

- **Fix:** Correct remapped generic handling in generated code ([1954594](https://github.com/vic1707/pud/commit/1954594)).
- **Chore:** Clean up test warnings ([184a072](https://github.com/vic1707/pud/commit/184a072)).

## [v0.0.2] - 2025-12-24

- **Feature:** Add the `macros` feature flag ([9c73c7a](https://github.com/vic1707/pud/commit/9c73c7a)).
- **Improvement:** Add a clippy allow attribute to the generated enum ([4d6005c](https://github.com/vic1707/pud/commit/4d6005c)).

## [v0.0.1] - 2025-12-21

Initial Release
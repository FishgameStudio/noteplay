# Contributing Guide

Thanks for your contributions. Please follow the steps below.

## 1. Coding Standards

This project follows Rust community conventions. The rules below supplement `rustfmt` and `clippy`.

### General

- Run `cargo fmt` before submitting PRs to format your code.
- Run `cargo clippy` and fix all warnings. Do not add `#[allow(...)]` unless you have a clear reason and comment it.
- Prefer safe Rust. `unsafe` must have a `// SAFETY:` comment explaining why it is sound.
- Keep functions small and focused. Avoid giant functions.
- Name things clearly; avoid abbreviations unless they are widely understood (e.g. `id`, `ctx`).
- If you created a new folder, please add a `README.md` file in it.
- Add unit tests for new APIs if possible.

### Error Handling

- Use the crate's top-level `Error` type for public APIs.
- Prefer `?` over manual `match` for error propagation.

### API Design

- Public items require doc comments (`///`).
- Encode invariants into types, avoid runtime precondition checks.
- Use `&self` / `&mut self` / `self` to express ownership clearly.
- Return `Result` for recoverable errors; use dedicated error enums.
- Keep traits small; use `#[non_exhaustive]` for semver safety.
- Hide unsafe, `Rc`, `RefCell` from public surfaces.

## 2. Commit Standards

Please use `gitmoji-cli` to commit. To install `gitmoji-cli`, please run:
```sh
npm install -g gitmoji-cli
# If you don't have npm, use cargo:
# cargo install gimoji
```

Please explain the title and the detail clearly, and don't add too many changes into one single commit.

## 3. Opening a Pull-Request

Write a concise title, fill in this template, and add tags if applicable.

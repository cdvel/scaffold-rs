# scaffold-rs

A custom template for [cargo generate](https://github.com/cargo-generate/cargo-generate) based off [rust-github/template](https://github.com/rust-github/template)


## Enhancements

- Structured test organization with examples of unit, integration, and doc tests

```
scaffold-rs/template/
├── src/
│   └── lib.rs           (with unit tests, doc tests)
├── tests/
│   ├── common/
│   │   └── mod.rs       (shared test utilities, setup/teardown)
│   ├── integration_tests.rs (test public API)
│   └── test_with_setup.rs
└── Cargo.toml           (with dev-dependencies)
```


- Pre-commit hooks configuration for code quality enforcement
- Rust version specification via `.rust-version` file

## Usage

Install cargo-generate if you don't have it already:

```bash
cargo install cargo-generate
```

Generate a new project:

```bash
cargo generate --git https://github.com/cdvel/scaffold-rs
```

Follow the prompts to configure your project.

## Available Template Options

- **Project type:** Binary or library
- **GitHub username:** For repository links and documentation
- **Project description:** For Cargo.toml and documentation

## License

MIT

# scaffold-rs

A comprehensive, opinionated Rust project template for [cargo generate](https://github.com/cargo-generate/cargo-generate) that provides a robust foundation for both library and binary crates. This template extends [rust-github/template](https://github.com/rust-github/template) with additional structure and features to support a wide range of development use cases - from simple utilities to complex applications with async capabilities, configuration management, and structured error handling.

Please review [ARCHITECTURE.md](ARCHITECTURE.md) for details about this template.

To learn more about the details of how all of these pieces fit together, please consult this ARCHITECTURE.md document

## Enhancements

- **Structured test organization**: with examples of unit, integration, and doc tests

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

- **Pre-commit hooks**: for code quality enforcement
- **Rust version explicit**: specification via `.rust-version` file
- **Modern Error Handling**: Built-in `thiserror` and `anyhow` integration with error type definitions
- **Command-Line Argument Parsing**: Ready-to-use CLI setup with `clap` derive feature
- **Configuration Management**: Flexible config system using `config` and `serde` with environment variable support
- **Structured Logging**: Preconfigured `tracing` setup for production-ready logging
- **Convenient Makefile**: Common commands for build, test, lint, and documentation

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

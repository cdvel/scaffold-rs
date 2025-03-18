# Architecture Overview

This document outlines the architectural approach used in this project template. The structure follows modern Rust best practices with a focus on modularity, separation of concerns, and maintainability.
It is designed to scale, starting simple and have a clean organization that can be grown if the project requires it.

## Rust File Structure Guide

| File           | Development Use Case                                    | When to Use                                      | Key Components                                       |
|----------------|--------------------------------------------------------|--------------------------------------------------|-----------------------------------------------------|
| `lib.rs`       | Shared business logic, core functionality              | Creating reusable components, library crates      | Public API, modules exports, domain logic, unit tests |
| `main.rs`      | Application entry point                                | Creating executable programs                      | Minimal bootstrap code, error handling, CLI parsing   |
| `app.rs`       | Application state and coordination                     | When you need central application management      | `App` struct, application lifecycle methods          |
| `cli.rs`       | Command-line interface                                 | When accepting user input via command line        | `Args` struct with `clap` attributes                 |
| `config.rs`    | Configuration management                               | When app needs configurable settings              | Config loading, validation, environment integration  |
| `error.rs`     | Custom error handling                                  | When standard errors aren't sufficient            | Error enums with `thiserror`, custom error messages  |
| `logging.rs`   | Logging and tracing setup                              | Production applications that need observability   | Logging initialization, log level configuration      |
| `async_utils.rs` | Asynchronous operations                              | Applications with I/O or concurrency needs       | Async functions, Tokio integration                   |
| `time_utils.rs` | Date and time handling                                | Applications that work with timestamps            | Temporal calculations, formatting, parsing           |
| `models/`      | Data structures and database models                    | Applications with complex data representations    | Domain entities, database mappings, validations      |
| `api/`         | API endpoints and handlers                             | Web services and API-driven applications          | Route definitions, request/response handlers         |
| `utils/`       | Miscellaneous helper functions                         | Common utilities needed across the application    | Helper functions, extension traits                   |
| `tests/`       | Integration and end-to-end tests                       | Testing across module boundaries                  | Test fixtures, integration tests, benchmarks         |

```mermaid
graph TD
    main[main.rs] --> |bootstraps| app[app.rs]
    main --> |parses args| cli[cli.rs]
    main --> |initializes| logging[logging.rs]
    app --> |uses core logic| lib[lib.rs]
    app --> |loads settings| config[config.rs]
    app --> |handles errors| error[error.rs]
    lib --> |imports| models[models/]
    lib --> |uses| utils[utils/]
    lib --> |may import| async[async_utils.rs]
    lib --> |may import| time[time_utils.rs]
    lib --> |defines| api[api/]
    error --> |used by| lib
    error --> |used by| app
    config --> |may use| error

    subgraph "Entry Point"
        main
        cli
    end

    subgraph "Application Core"
        app
        config
        logging
        error
    end

    subgraph "Business Logic"
        lib
        models
        utils
        async
        time
        api
    end

    tests[tests/] --> |tests| lib
    tests --> |tests| app

    style main fill:#f9d77e
    style lib fill:#a4e1b4
    style app fill:#a4d0e1
    style error fill:#e1a4a4
    style tests fill:#d8a4e1
```

## Tests structure

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

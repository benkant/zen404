# 404zen

This project uses `cargo xtask` for build and development automation.

## Prerequisites

Ensure you have Rust and Cargo installed. Some tasks may require additional tools (e.g., `taplo`, `pre-commit`, `cargo-about`, `quicktype`, `fuckup`). These should be installed (often via `cargo install <tool_name>`) and their binaries accessible (typically via `$CARGO_HOME/bin` or `$HOME/.cargo/bin`).

## Available `xtask` Commands

You can run these commands from the root of the project using `cargo xtask <command>`.

*   `cargo xtask check`: Runs `cargo check --all-targets --all-features`.
*   `cargo xtask clippy`: Runs `cargo clippy --all-targets --all-features -- -D warnings`.
*   `cargo xtask doc [--open]`: Runs `cargo doc --no-deps`. Use `--open` to open the documentation in your browser.
*   `cargo xtask build`: Runs `cargo build --all-targets --all-features`.
*   `cargo xtask test`: Runs `cargo test --all-targets --all-features --verbose`.
*   `cargo xtask taplo-format`: Formats all TOML files in the project using `taplo format .`.
*   `cargo xtask taplo-lint`: Lints all TOML files in the project using `taplo lint .`.
*   `cargo xtask pre-commit-run`: Runs `pre-commit run --all-files`. (Requires `pre-commit` to be installed and hooks configured).
*   `cargo xtask pre-commit-install`: Runs `pre-commit install`. (Requires `pre-commit` to be installed).
*   `cargo xtask cargo-about`: Checks license compliance using `cargo about generate --format json -o /dev/null` against `about.toml`.
*   `cargo xtask quicktype <input.json>`: Generates Rust types from a given JSON input file using `quicktype`.
*   `cargo xtask mistake <mistake_report.json>`: Processes a mistake report. (This task currently relies on a command named `fuckup` which must be available).
*   `cargo xtask all` or `cargo xtask ci`: Runs a comprehensive suite of CI checks: `taplo-format`, `taplo-lint`, `clippy`, `test`, `check`, `doc`, `cargo-about`, and `pre-commit-run`.

For a full list of commands and detailed options, you can also check the `xtask/src/main.rs` file or run `cargo xtask` with no arguments or an invalid command to see the help message.

## Building and Running

*   To build the project: `cargo build`
*   To run tests: `cargo test`
*   To run the main binary: `cargo run --bin zen404` (if applicable)
*   To run the fuckup binary: `cargo run --bin fuckup` (if applicable)

## License

This project is licensed under the terms of the ISC license. See the LICENSE file for details.

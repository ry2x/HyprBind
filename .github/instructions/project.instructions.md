## Project Scope (For AI Agents)

This project builds a **GUI application to inspect Hyprland keybindings**.

Primary goal:

* Accurately parse, represent, and display Hyprland keybinds
* Provide a fast, minimal, non-bloated GUI for inspection and filtering

This is **not** a general-purpose config manager.
This is **not** a theming playground.
If functionality does not directly support keybind inspection, question it.

## Technology Constraints (Non-Negotiable)

### Language

* Rust (2024 edition)
* **Stable channel only**
* No nightly features
* No experimental compiler flags

### Tooling (Defined in `/rust-toolchain.toml`)

Required usage:

* `rustfmt` — formatting (no manual style deviations)
* `clippy` — linting (warnings treated as design feedback)
* `rust-analyzer` — LSP
* `rust-docs`
* `rust-src`
* `cargo-audit` — dependency security

Ignoring these tools is considered a defect, not a preference.

## Dependency & Framework Choices

* Dependency management: **Cargo only**
* GUI:

  * `egui` — immediate mode GUI
  * `eframe` — application framework
* CLI argument parsing: `clap`
* Testing: Rust built-in test framework
* CI/CD: GitHub Actions

Do **not** introduce alternative UI frameworks, async runtimes, or CLI parsers without explicit justification.

## Code Quality Rules

* Formatting: `cargo fmt` (mandatory)
* Linting: `cargo clippy` (fix, don’t silence)
* Tests: required for logic-heavy components
* Security: `cargo audit` must pass

If code “works” but violates these, it is still wrong.

## Standard Commands (Expected Knowledge)

* Run (dev): `cargo run`
* Build (dev): `cargo build`
* Build (release): `cargo build --release`
* Format: `cargo fmt`
* Lint: `cargo clippy`
* Test: `cargo test`
* Security audit: `cargo audit`

Do not suggest alternative workflows unless they improve leverage measurably.

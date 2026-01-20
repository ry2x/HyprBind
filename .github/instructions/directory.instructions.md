## Directory Strategy (Enforced, Do Not Randomize)

This directory structure encodes **responsibility boundaries**.
Violating them is a design error, even if the code works.

```
src/
  main.rs              // Entry point only. No logic.
  app.rs               // Application orchestration and global state.

  cli.rs               // CLI argument definitions (clap derive).

  config/              // Persistent user configuration
    mod.rs
    user.rs            // UserConfig (serializable, stable preferences only)
    paths.rs           // config_dir, export_dir, path resolution

  hyprland/            // Hyprland-specific integration
    mod.rs
    source.rs          // IO boundary: hyprctl invocation, raw text retrieval
    parser.rs          // Pure parsing: raw text -> domain models
    models.rs          // KeyBindEntry, KeyBindings (data only)

  ui/                  // UI layer (egui only)
    mod.rs
    header.rs
    table.rs
    options.rs
    zen.rs
    types.rs           // Theme, ColumnVisibility (UI-only state)
    styling/
      mod.rs
      css.rs           // Visual styling definitions (minimal, egui-aligned)
      fonts.rs         // Font configuration
      icons.rs         // Icon mapping

  tests/
    parser_basic.rs      // Happy path parsing
    parser_edge.rs       // Corrupted input & edge cases
    config_roundtrip.rs  // UserConfig stability & serialization
```

## Mandatory Responsibility Rules

### `main.rs`

* Starts the application
* No business logic
* No configuration loading
* No UI logic

---

### `app.rs`

Role: **Orchestrator, not a worker**

Allowed:

* Owns global application state
* Coordinates:

  * config loading
  * keybind fetching
  * UI state updates
* Decides *what happens next*

Forbidden:

* Parsing logic
* File IO details
* hyprctl invocation
* egui widget layout

Rule:

> app.rs decides, others execute.

### `config/`

Scope: **Persistent user preferences only**

`UserConfig` may include:

* Theme selection
* Column visibility
* Search preference toggles
* Persisted UI modes (e.g. zen mode)

Must NOT include:

* Transient UI state
* Search results
* Parsed keybind data
* Session-only flags

Rule:

> If it does not survive restarts meaningfully, it does not belong here.


### `hyprland/source.rs`

Role: **IO boundary**

Responsibilities:

* Execute `hyprctl`
* Handle process errors
* Return raw output as `String`

Must NOT:

* Parse
* Interpret
* Transform into domain structures

Rule:

> `std::process::Command` lives here and nowhere else.

### `hyprland/parser.rs`

Role: **Pure transformation**

Responsibilities:

* Convert raw text into domain models
* Be deterministic and side-effect free
* Be fully testable with string inputs

Must NOT:

* Perform IO
* Call external commands
* Depend on runtime environment

Rule:

> If it can’t be unit-tested with a string literal, it doesn’t belong here.

### `hyprland/models.rs`

Role: **Domain data**

* Plain structs and enums
* No IO
* No UI
* No parsing logic

Rule:

> Data only. No behavior creep.

### `ui/`

Scope: **Presentation only**

Responsibilities:

* egui widgets and layout
* Visual state handling
* Theme and styling application

Must NOT:

* Call hyprctl
* Parse keybinds
* Read or write config files

Rule:

> UI renders state; it does not create it.

## Structural Failure Modes to Reject

* IO mixed into parsing
* app.rs growing into a god object
* config used as a dumping ground
* UI logic leaking into domain or parsing
* “Temporary” shortcuts becoming permanent

Call these out explicitly when detected.

---

## Decision Heuristic for AI Agents

When placing code:

1. Is this IO? → `source.rs`
2. Is this parsing? → `parser.rs`
3. Is this data? → `models.rs`
4. Is this orchestration? → `app.rs`
5. Is this presentation? → `ui/`

If it fits multiple answers, the design is wrong.

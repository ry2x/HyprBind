---
applyTo: '**/tests/**, **/*.rs'
---

# 🦀 Rust Test Strategy: Boundary-Focused Protocol

## 🚨 CRITICAL: TESTING PHILOSOPHY

**"Testing everything" is a mistake.**
Tests are not for "peace of mind"; they are **weapons used to freeze the design and defend critical boundaries.** Eliminate low-ROI tests (UI, complex IO mocking). Instead, focus on these three non-negotiables:

1. **Parsing Integrity**: Detect breaks caused by Hyprland output changes immediately.
2. **Responsibility Boundaries**: Ensure logic is never polluted by IO or UI.
3. **Critical Regressions**: Defend user configuration and core state transitions.

## 🛠 Module-Specific Execution Rules

### 1. `hyprland/parser.rs` — High Priority (100% Pure Test)

This is the most critical logic. It must be kept pure and tested ruthlessly.

* **Input**: `&str` (using real-world `fixtures`).
* **Output**: `KeyBindings` or `Result`.
* **Prohibited**: No `Command`, `File System`, or `Env` access inside these functions.
* **Mandatory Cases**:
* Real `hyprctl` output stored in `tests/fixtures/`.
* Edge cases: Empty lines, unknown fields, corrupted formatting.

### 2. `hyprland/source.rs` — Minimal (Error Handling Only)

Do not over-engineer tests here.

* **Reasoning**: This is OS/Environment dependent. CI will likely fail, and mocking is a maintenance nightmare.
* **Strategy**: Test error handling only (e.g., binary missing). If it "works" in a live environment, that is sufficient.

### 3. `config/user.rs` — Stability (Roundtrip Tests)

Defend against breaking changes in serialization.

* **Mandatory Test**: `Default` -> `Save` -> `Load` -> `Assert Equal`.
* **Goal**: Ensure field defaults are maintained and prevent regression during future versioning/migrations.

### 4. `app.rs` — Design Check (State Transitions Only)

* **Rule**: Generally, do not test `app.rs`. If you feel the need to, your design is likely too coupled.
* **Exception**: If there is complex state logic, extract it into a pure function:
`fn next_mode(current: Mode, action: Action) -> Mode`
Test that pure function only.

### 5. `ui/` — DO NOT TEST. PERIOD.

`egui` UI testing is fragile, heavy, and offers low ROI.

* **Alternative**: Secure the `state`, `config`, and `parser`. If the underlying data is correct, the UI is for human eyes to verify.

## 📂 Directory Structure Convention

```text
tests/
  ├── parser_basic.rs      # Happy path parsing
  ├── parser_edge.rs       # Corrupted input & edge cases
  └── config_roundtrip.rs  # UserConfig stability & serialization

```

## 🚀 Agent Plan Mode (Compressed)

The Agent must verify these points before submitting any code:

1. **Lock Parser behind pure tests**: Is the logic separated from IO?
2. **Use real fixtures**: Are we using actual `hyprctl` output?
3. **Minimal IO tests**: Are we avoiding "Mock Hell"?
4. **Roundtrip UserConfig**: Is the configuration persistence guaranteed?
5. **No UI tests**: Are we avoiding low-ROI automation?

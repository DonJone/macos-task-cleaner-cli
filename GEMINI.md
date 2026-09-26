# macOS Task Cleaner CLI - Agent Guidelines & Engineering Constraints

This document defines the architectural conventions, engineering rules, and hard constraints for AI coding agents operating on the `macos-task-cleaner-cli` (`mtc`) codebase.

---

## 1. Global Operating Policies

1. **Strict No-Emoji Policy**:
   * Never output Unicode emojis in code, comments, Git commit messages, logs, UI strings, documentation, or responses.
   * Use plain text prefixes for emphasis or status (e.g., `[INFO]`, `[WARN]`, `[SUCCESS]`, `*`, `-`).

2. **Workspace Delivery Principle**:
   * All deliverables, source code modifications, scripts, and documentation must physically persist within the local workspace directory.
   * Never leave deliverables exclusively in hidden cache directories.

3. **Clickable File Links**:
   * All file paths and symbol references in explanations must use the `file://` scheme (e.g. `file:///Users/don/work/git/macos-task-cleaner-cli/src/main.rs`).

---

## 2. Architecture Overview

`mtc` (or `taskcleaner`) is a fast, keyboard-driven CLI utility built on top of `macos-task-cleaner-core`.

* `src/main.rs`: CLI entry point, argument parsing via `clap 4.x`, interactive terminal wizard, and JSON serialization.
* `Cargo.toml`: Package configuration, dependencies (`macos-task-cleaner-core`, `clap`, `colored`, `serde`, `serde_json`, `toml`).

---

## 3. Engineering Constraints & Rules

### A. Terminal Interface & Wizard Design
* **Interactive Mode (`-i` / `--interactive`)**: Pure keyboard-driven console workflow. Must support numeric index actions, instant whitelisting, skipping, and bulk execution.
* **Pre-Flight Mode (`-n` / `--dry-run`)**: Must scan and report status without dispatching any POSIX kill signals.
* **JSON Schema Stability (`--json`)**: Third-party integrations (e.g., Raycast extensions, Alfred workflows, shell scripts) parse `mtc --json --dry-run`. Always preserve backwards compatibility of the JSON output schema (`scanned_total`, `protected_count`, `target_count`, `targets`, `protected_apps`).

### B. Attribution & Licensing Watermarks
* The CLI `--version` (`-v`) and `--help` (`-h`) flags must always print:
  * Version: `v0.1.0`
  * Copyright: `Copyright (c) 2026 DonJone. All rights reserved.`
  * License: `GNU AGPLv3 / Commercial Dual License`
  * Direct links to `COMMERCIAL.md` and `TRADEMARK.md`.

### C. Build & Verification
* Local development: `cargo check` and `cargo test`.
* Standalone binary compilation: `cargo build --release`.

---

## 4. Licensing & Commercial Policy

* **Dual-Licensing Model**:
  * Open-source under **GNU AGPLv3**.
  * Commercial closed-source bundling or white-labeling requires a separate commercial license.
* **Documentation**:
  * Maintain `LICENSE`, `COMMERCIAL.md`, and `TRADEMARK.md` at repository root.

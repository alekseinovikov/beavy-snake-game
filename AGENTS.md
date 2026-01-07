# Repository Guidelines

## Project Structure & Module Organization
- Workspace root is `Cargo.toml`; crates live under `crates/`.
- `crates/app/` is the binary crate (`beavy-snake-game`) that calls `engine::run()`.
- `crates/engine/` owns Bevy (v0.17.3) setup, window config, and FPS UI overlay.
- `crates/game/` holds engine-agnostic state (`GameState`) and update logic.

## Build, Test, and Development Commands
- `cargo run -p beavy-snake-game` runs the app (spawns a 960x540 window with FPS in top-left).
- `cargo build -p beavy-snake-game` compiles the app in debug mode.
- `cargo build --release` builds optimized binaries in `target/release/`.
- `cargo fmt` formats code (rustfmt is installed via `rust-toolchain.toml`).
- `cargo clippy` runs the linter (component is pinned in `rust-toolchain.toml`).
- `cargo test` runs tests (none exist yet).

## Coding Style & Naming Conventions
- Rust 2024 edition; keep formatting via `cargo fmt`.
- Naming: `snake_case` for functions/variables, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Keep modules small; keep Bevy-specific code inside `crates/engine/`.

## Testing Guidelines
- Run `cargo clippy` and `cargo test` for every change you make.
- Add unit tests beside code in `src/` with `#[cfg(test)] mod tests`.
- Create `tests/` for integration tests when gameplay modules grow.

## Commit & Pull Request Guidelines
- No enforced convention; use clear, imperative messages (e.g., "Add snake movement system").
- PRs should include a concise summary, motivation, and manual test steps (e.g., `cargo run -p beavy-snake-game`).

# Contributing to SetlisteR

First off, thank you for considering contributing to SetlisteR! It's people like you that make this tool great. We welcome all contributions — from reporting bugs to suggesting new features, improving documentation, or submitting code changes.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [How to Get Help / Ask Questions](#how-to-get-help--ask-questions)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [First Code Contribution](#first-code-contribution)
  - [Improving Documentation](#improving-documentation)
- [Local Development Setup](#local-development-setup)
- [Project Structure](#project-structure)
- [Testing Guidelines](#testing-guidelines)
- [Test Pattern](#test-pattern)
- [Pull Request Process](#pull-request-process)
- [AI Guidelines](#ai-guidelines)
- [Styleguides](#styleguides)
- [Recognition](#recognition)

## Code of Conduct
By participating in this project, you agree to abide by our behavioral expectations. Please be respectful and considerate of others. If a `CODE_OF_CONDUCT.md` file is present in the repository, please review it.

## How to Get Help / Ask Questions
If you have a question about how to use SetlisteR or how to contribute, please check the existing GitHub Discussions or open a new Discussion. Please do not use the issue tracker for support questions to help us keep it focused on bugs and feature requests.

## How to Contribute

### Reporting Bugs
Bugs are tracked as GitHub issues. When reporting a bug, please use the provided Bug Report template (if available) or include the following:
- **Operating System** and version.
- **Rust version** (`rustc --version`).
- **Steps to reproduce** the behavior.
- **Expected behavior** and what actually happened.
- Any relevant logs or output.

### Suggesting Enhancements
Enhancement suggestions are also tracked as GitHub issues. Please use the Feature Request template if provided. Describe the feature you would like to see, why you need it, and how it should work.

### First Code Contribution
If you are looking for a place to start, check out the issue tracker for issues labeled `good first issue` or `help wanted`. These are a great way to get familiar with the codebase.

### Improving Documentation
Documentation improvements are always welcome! This includes updating the `README.md`, fixing typos, or adding comments to the codebase. Documentation pull requests follow the same process as code contributions.

## Local Development Setup

### Prerequisites
- **Rust** (edition `2024` — use [rustup](https://rustup.rs) to install/manage).
- **[uv](https://docs.astral.sh/uv/)** — manages the Python toolchain and virtualenv for the YouTube Music sidecar. `cargo build` invokes `uv run pyinstaller` automatically via `build.rs`, so it must be available on your `PATH`.
- On **Linux**: `libdbus-1-dev` and `pkg-config` (required by the keyring crate).
  ```sh
  sudo apt-get install -y libdbus-1-dev pkg-config
  ```

### Steps

1. **Clone the repository:**
   ```sh
   git clone https://github.com/timothyspriegade/SetlisteR.git
   cd SetlisteR
   ```

2. **Sync the Python sidecar dependencies:**
   This installs `pyinstaller` and `ytmusicapi` into a local virtualenv under `ytmusic_sidecar/.venv`.
   ```sh
   cd ytmusic_sidecar
   uv sync
   cd ..
   ```

3. **Set up credentials:**
   SetlisteR resolves secrets in this order:
   1. System keyring entry (service: `SetlisteR`)
   2. Environment variable fallback: `SETLIST_FM_API_KEY`

   Store the key in the keyring on first run:
   ```sh
   cargo run -- --artists "Radiohead" --service spotify --setlist-api-key "<YOUR_SETLIST_FM_API_KEY>"
   ```
   Or export the fallback env var:
   ```sh
   export SETLIST_FM_API_KEY="<YOUR_SETLIST_FM_API_KEY>"
   ```

4. **Build the project:**
   `cargo build` automatically compiles the Python sidecar via `build.rs` before linking the Rust binary.
   ```sh
   cargo build
   ```

## Project Structure

```
SetlisteR/
├── src/
│   ├── main.rs              # Entry point — argument parsing and top-level dispatch
│   ├── api/                 # setlist.fm API client (HTTP, pagination, rate-limiting)
│   ├── data/                # Data models and serde serialization/deserialization
│   ├── playlist_creator/    # Ranking logic and playlist assembly
│   ├── secrets_manager/     # Keyring-backed credential storage and retrieval
│   └── validator/           # Input validation for artists, playlist names, and args
├── ytmusic_sidecar/         # Python sidecar compiled into a self-contained binary by build.rs
│   ├── main.py              # CLI entry point (subcommands: login, create-playlist)
│   ├── auth.py              # Browser header sanitisation and ytmusicapi setup
│   ├── playlist.py          # Song search and playlist creation via ytmusicapi
│   └── pyproject.toml       # Python project metadata and dependency declarations
├── build.rs                 # Cargo build script — invokes PyInstaller to compile the sidecar
└── Cargo.toml
```

## Testing Guidelines
We expect all code contributions to pass the existing test suite and include new tests for new functionality.

Run the full test suite:
```sh
cargo test
```

Please ensure all tests pass locally before opening a pull request. If you are adding a new feature or fixing a bug, please add corresponding unit or integration tests to verify your changes.

## Test Pattern
When adding tests to SetlisteR, please adhere to the following patterns:

- **Unit Tests:** Place unit tests in the same file as the code they are testing, wrapped in a `#[cfg(test)]` module named `tests`. Keep tests small and focused on a single piece of functionality.
- **Mocking:** Where appropriate, use mocks for external services like the setlist.fm API to ensure tests remain fast and deterministic without relying on network calls.
- **Test Data (Object Mother):** You **must** use the Object Mother pattern to create standardised, reusable test data fixtures (such as dummy arguments, API payloads, or setlists) rather than hardcoding complex data structures directly inside individual tests.

## Pull Request Process
1. **Fork the repository** and create your branch from `main`.
2. **Commit your changes** with clear, descriptive commit messages.
3. **Ensure tests and linting pass** locally (see [Styleguides](#styleguides)).
4. **Open a Pull Request** describing your changes and referencing any related issues.
5. **Review:** A maintainer will review your PR. You may be asked to make changes before it can be merged.

## AI Guidelines
We welcome the use of AI tools (like GitHub Copilot, ChatGPT, Claude, etc.) to assist with your development. However, please adhere to the following guidelines when submitting AI-assisted contributions:

- **Manual Review Required:** You must manually review, understand, and thoroughly test any AI-generated code before submitting. Do not blindly copy-paste AI outputs.
- **Scope of Changes:** Extremely large or sweeping AI-generated changes will likely be declined. Keep contributions focused, properly scoped, and easy to review.

## Styleguides

### Rust
Run these before every commit:

```sh
# Format
cargo fmt --all

# Lint (warnings are errors)
cargo clippy -- -D warnings
```

### Python (ytmusic_sidecar)
The sidecar follows standard Python conventions. If you modify any `.py` files, run:

```sh
cd ytmusic_sidecar

# Format
uv run ruff format .

# Lint
uv run ruff check .
```

> **Note:** `ruff` is not currently listed as a dev dependency. Add it with `uv add --dev ruff` if you plan to contribute to the sidecar.

## Recognition
<a href="https://github.com/timothyspriegade/SetlisteR/graphs/contributors">
<img src="https://contrib.rocks/image?repo=timothyspriegade/SetlisteR" />
</a>

We appreciate all contributions! Thank you for helping make SetlisteR better.
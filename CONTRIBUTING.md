# Contributing to SetlisteR

First off, thank you for considering contributing to SetlisteR! It's people like you that make this tool great. We welcome all contributions, from reporting bugs to suggesting new features, improving documentation, or submitting code changes.

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
*   **Operating System** and version.
*   **Rust version** (`rustc --version`).
*   **Steps to reproduce** the behavior.
*   **Expected behavior** and what actually happened.
*   Any relevant logs or output.

### Suggesting Enhancements
Enhancement suggestions are also tracked as GitHub issues. Please use the Feature Request template if provided. Describe the feature you would like to see, why you need it, and how it should work.

### First Code Contribution
If you are looking for a place to start, check out the issue tracker for issues labeled `good first issue` or `help wanted`. These issues are a great way to get familiar with the codebase.

### Improving Documentation
Documentation improvements are always welcome! This includes updating the `README.md`, fixing typos, or adding comments to the codebase. Documentation pull requests follow the same process as code contributions.

## Local Development Setup
To set up SetlisteR for local development, follow these steps:

1. **Clone the repository:**
   ```sh
   git clone https://github.com/your-username/SetlisteR.git
   cd SetlisteR
   ```

2. **Set up the environment:**
   SetlisteR requires a setlist.fm API key. Secrets are resolved in this order:
   1. System keyring entry (service: `SetlisteR`, user: `setlist_fm_api_key`)
   2. Environment variable fallback: `SETLIST_FM_API_KEY`

   Store the key in keyring on first run:
   ```sh
   cargo run -- --artists "Radiohead" --service spotify --setlist-api-key "<YOUR_SETLIST_FM_API_KEY>"
   ```

   Or export the fallback env var:
   ```sh
   export SETLIST_FM_API_KEY="<YOUR_SETLIST_FM_API_KEY>"
   ```

3. **Build the project:**
   Make sure you have Rust installed (edition `2024` is required; see `Cargo.toml`). We recommend using `rustup`.
   ```sh
   cargo build --release
   ```

## Project Structure
Here's a brief overview of our standard Rust project layout:
- `src/main.rs`: The main entry point parsing arguments and calling the API.
- `src/api/`: Logic for handling API requests (like the setlist.fm client). This directory is strictly for API communication.
- `src/data/`: Data structures and models for serialization/deserialization.
- `src/secrets_manager/`: Logic utilizing the `keyring` crate to securely store and retrieve API credentials natively.
- `src/validator/`: Contains modules for validating input arguments, artists, and playlist names.

## Testing Guidelines
We expect all code contributions to pass the existing test suite and include new tests for new functionality.

To run the standard test suite:
```sh
cargo test
```

Please ensure that all tests pass locally before opening a pull request. If you are adding a new feature or fixing a bug, please add corresponding unit or integration tests to verify your changes.

## Test Pattern
When adding tests to SetlisteR, please adhere to the following patterns:
- **Unit Tests:** Place unit tests in the same file as the code they are testing, wrapped in a `#[cfg(test)]` module named `tests`. Keep tests small and focused on a single piece of functionality.
- **Mocking:** Where appropriate, use mocks for external services like the setlist.fm API to ensure tests remain fast and deterministic without relying on network calls.
- **Test Data (Object Mother):** You **MUST** use the Object Mother pattern to create standardized, reusable test data fixtures (such as dummy arguments, API payloads, or setlists) rather than hardcoding complex data structures directly inside individual tests.

## Pull Request Process
1. **Fork the repository** and create your branch from `main`.
2. **Commit your changes** with clear, descriptive commit messages (no enforced commit-message convention is currently configured in the repository).
3. **Ensure tests and linting pass** locally (see Testing Guidelines and Styleguides).
4. **Open a Pull Request** describing your changes, referencing any related issues.
5. **Review:** A maintainer will review your PR. You may be asked to make changes before it can be merged.

## AI Guidelines
We welcome the use of AI tools (like GitHub Copilot, ChatGPT, Claude, etc.) to assist with your development! However, please adhere to the following strict guidelines when submitting AI-assisted contributions:
- **Manual Review Required:** You must manually review, understand, and thoroughly test any code generated by AI before submitting it. Do not blindly copy-paste AI outputs.
- **Scope of Changes:** Complete and extremely large or sweeping AI-generated changes will likely be ignored or declined. Please keep your contributions focused, properly scoped, and easy to review.

## Styleguides
We adhere to standard Rust coding conventions. Before submitting a pull request, please ensure your code follows these formatting and linting rules:

*   **Formatting:** Run `rustfmt` to format your code.
    ```sh
    cargo fmt --all
    ```
*   **Linting:** Run `clippy` to catch common mistakes and improve your code.
    ```sh
    cargo clippy -- -D warnings
    ```

## Recognition
<a href="https://github.com/timothyspriegade/SetlisteR/graphs/contributors">
<img src="https://contrib.rocks/image?repo=timothyspriegade/SetlisteR" />
</a>

We appreciate all contributions! Core contributors and community members who submit valuable PRs, report critical bugs, or help others in the community will be recognized. Thank you for helping make SetlisteR better!
<h1>
</h1>
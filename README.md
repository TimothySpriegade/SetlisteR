# SetlisteR

> A CLI tool that fetches live setlist data from [setlist.fm](https://www.setlist.fm) and builds a playlist on your
> favourite streaming service.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
    - [Arguments](#arguments)
    - [Examples](#examples)
- [How It Works](#how-it-works)
- [Project Structure](#project-structure)
- [Running Tests](#running-tests)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

SetlisteR takes one or more artist names, queries the setlist.fm API for their most recent live setlists, and generates
a playlist on a supported streaming service — so you can listen along before (or after) a show.

---

## Features

- **Multi-artist support** — query up to 10 artists in a single run
- **Configurable page depth** — control how many pages of setlist history are fetched per artist
- **Streaming service selection** — target Spotify or YouTube Music
- **Smart input sanitisation** — strips non-ASCII characters, collapses whitespace, deduplicates artists
- **Auto-generated playlist names** — falls back to a sensible name when none is provided
- **Async & rate-limit aware** — built on Tokio, respects the setlist.fm 2 req/s limit

---

## Prerequisites

| Requirement                                               | Version               |
|-----------------------------------------------------------|-----------------------|
| [Rust](https://www.rust-lang.org/tools/install)           | 1.85+ (edition 2024)  |
| [setlist.fm API key](https://www.setlist.fm/settings/api) | Free account required |

---

## Installation

```bash
# Clone the repository
git clone https://github.com/your-username/SetlisteR.git
cd SetlisteR

# Build a release binary
cargo build --release

# The binary will be at
./target/release/SetlisteR
```

---

## Configuration

SetlisteR uses a `.env` file to manage secrets. Copy the provided template and fill in your key:

```bash
cp .env.template .env
```

Open `.env` and replace the placeholder with your actual setlist.fm API key:

```dotenv
SETLIST_FM_API_KEY=your_super_private_setlist_fm_api_key_here
```

> **Never commit your `.env` file.** It is already listed in `.gitignore`.

You can obtain a free API key by creating an account at [setlist.fm](https://www.setlist.fm/settings/api).

---

## Usage

```bash
./target/release/SetlisteR [OPTIONS] --artists <ARTISTS >--service <SERVICE>
```

### Arguments

| Flag              | Short | Type                          | Required | Default        | Description                                   |
|-------------------|-------|-------------------------------|----------|----------------|-----------------------------------------------|
| `--artists`       | `-a`  | `String`                      | ✅        | —              | Comma-separated list of artist names (max 10) |
| `--service`       | `-s`  | `spotify` \| `you-tube-music` | ✅        | —              | Streaming service to target                   |
| `--playlist-name` | `-p`  | `String`                      | ❌        | Auto-generated | Name for the created playlist (max 100 chars) |
| `--page-depth`    | `-d`  | `u16`                         | ❌        | `1`            | Number of setlist pages to fetch per artist   |

### Examples

**Single artist, default playlist name:**

```bash
./target/release/SetlisteR --artists "Metallica" --service spotify
```

**Multiple artists with a custom playlist name:**

```bash
./target/release/SetlisteR \
  --artists "Metallica, Slayer, Megadeth" \
  --service spotify \
  --playlist-name "Big Four Night"
```

**Fetch more setlist history with page depth:**

```bash
./target/release/SetlisteR \
  --artists "Radiohead" \
  --service you-tube-music \
  --page-depth 5
```

---

## How It Works

```
CLI Args
   │
   ▼
ArgValidator
   ├── ArtistValidator   → sanitises names, deduplicates, enforces limits
   └── PlaylistNameValidator → trims, collapses whitespace, truncates if needed
   │
   ▼
SetlistFmClient
   └── GET /rest/1.0/search/setlists?artistName=<artist>&p=<page>
       (batched in pairs, 1 s sleep between batches to stay within 2 req/s)
   │
   ▼
Streaming service integration (coming soon)
```

### Rate limiting

The setlist.fm free tier allows **2 requests per second**. SetlisteR fetches pages sequentially, sleeping for 1 second
after every 2 pages, so it will never exceed that limit regardless of `--page-depth`.

### Input sanitisation rules

| Rule                                   | Artists        | Playlist name   |
|----------------------------------------|----------------|-----------------|
| Leading / trailing whitespace stripped | ✅              | ✅               |
| Inner whitespace collapsed             | ✅              | ✅               |
| Non-ASCII characters removed           | ✅              | ✅               |
| Duplicates removed                     | ✅              | —               |
| Empty entries removed                  | ✅              | —               |
| Max length enforced                    | 100 chars each | 100 chars total |
| Max count enforced                     | 10 artists     | —               |

---

## Project Structure

```
src/
├── main.rs                         # Entry point, CLI arg parsing
├── api/
│   ├── mod.rs
│   └── setlist_fm.rs               # setlist.fm HTTP client
└── validator/
    ├── mod.rs
    ├── arg_validator.rs            # Top-level validation orchestrator
    ├── artist_validator.rs         # Artist name sanitisation & validation
    ├── playlist_name_validator.rs  # Playlist name sanitisation & validation
    └── tests/
        ├── mod.rs
        ├── args_mother.rs          # ArgsMotherObject test fixture (builder pattern)
        ├── artists_validator.rs    # Artist validator tests
        └── playlist_name_validator.rs  # Playlist name validator tests
```

---

## Running Tests

```bash
cargo test
```

The test suite uses an **Object Mother** pattern (`ArgsMotherObject`) to build `Args` instances with sensible defaults
while allowing any field to be overridden via a fluent builder API:

```rust
let args = ArgsMotherObject::default ()
.with_artists("Metallica, Slayer")
.with_page_depth(3)
.build();
```

All tests follow the **Arrange / Act / Assert** structure.

---

## Contributing

Contributions are welcome! Please open an issue first to discuss what you'd like to change.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m 'Add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request

Bug reports and feature requests can be submitted via
the [issue tracker](https://github.com/your-username/SetlisteR/issues) using the provided templates.

---

## License

This project is open source. See [LICENSE](LICENSE) for details.




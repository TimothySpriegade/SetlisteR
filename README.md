# SetlisteR
> SetlisteR is a Rust command-line application that fetches artist setlists from setlist.fm and produces a ranked, setlist-shaped song list per artist.

## Features
- Fetches setlists from the setlist.fm REST API for one or more artists.
- Supports configurable pagination via `--page-depth`.
- Uses concurrent artist fetches while respecting a global request pace (`Duration::from_millis(500)`, approximately 2 requests/second).
- Computes per-song statistics including total plays, opener/closer/encore counts, average position, and last played date.
- Builds output playlists using role-aware ranking (opener/regular/closer/encore) and average setlist length.
- Supports secure API key storage in system keyring plus environment-variable fallback.
- Prints generated playlist output (playlist name, selected platform, and songs by position) to stdout.

## Prerequisites
- Rust toolchain that supports edition `2024`.
- Cargo.
- A valid setlist.fm API key.
- Optional but recommended: a working OS keyring backend (macOS Keychain, Windows Credential Manager, or Linux Secret Service-compatible backend).

## Installation
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd SetlisteR
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. (Optional) Run tests:
   ```bash
   cargo test
   ```

## Configuration
SetlisteR resolves the setlist.fm key in this order:
1. Keyring entry
2. Environment variable fallback (`SETLIST_FM_API_KEY`)

### Store API key in keyring
Run once with `--setlist-api-key`:

```bash
cargo run -- --artists "Radiohead" --service spotify --setlist-api-key "<YOUR_SETLIST_FM_API_KEY>"
```

The key is stored under:
- service: `SetlisteR`
- user: `setlist_fm_api_key`

### Use environment variable fallback
If keyring is unavailable (or empty), export:

```bash
export SETLIST_FM_API_KEY="<YOUR_SETLIST_FM_API_KEY>"
```

## Usage

### Arguments
`cargo run -- --help` currently reports:

```text
Usage: SetlisteR [OPTIONS] --artists <ARTISTS> --service <SERVICE>

Options:
  -a, --artists <ARTISTS>
  -p, --playlist-name <PLAYLIST_NAME>
  -s, --service <SERVICE>                  [possible values: spotify, you-tube-music]
      --page-depth <PAGE_DEPTH>            [default: 1]
      --setlist-api-key <SETLIST_API_KEY>  Store the setlist.fm API key in the system keyring
  -h, --help                               Print help
```

Additional validation behavior:
- `--artists` accepts a comma-separated list; duplicates are removed.
- Maximum artists: `10`.
- Maximum artist name length: `100` characters.
- `--playlist-name` max length: `100` characters; if omitted, a name is generated automatically.

### Examples
Store your API key in keyring on first run:

```bash
cargo run -- --artists "Radiohead" --service spotify --setlist-api-key "<YOUR_SETLIST_FM_API_KEY>"
```

Generate output for one artist:

```bash
cargo run -- --artists "Radiohead" --service spotify
```

Generate output for multiple artists and fetch deeper history:

```bash
cargo run -- --artists "Radiohead, Muse" --service you-tube-music --page-depth 3
```

Use a custom playlist name:

```bash
cargo run -- --artists "The National" --service spotify --playlist-name "Late Night Set"
```

## Contributing
Please read the contribution guide: [CONTRIBUTING.md](./CONTRIBUTING.md).

## License
This project is licensed under the terms described in the [License](./LICENSE).

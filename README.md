# SetlisteR
> SetlisteR is a Rust command-line application that fetches artist setlists from setlist.fm and produces a ranked, setlist-shaped song list per artist.

## Features
- Fetches setlists from the setlist.fm REST API for one or more artists.
- Supports configurable pagination via `--page-depth`.
- Uses concurrent artist fetches while respecting a global request pace (`Duration::from_millis(500)`, approximately 2 requests/second).
- Computes per-song statistics including total plays, opener/closer/encore counts, average position, and last played date.
- Builds output playlists using role-aware ranking (opener/regular/closer/encore) and average setlist length.
- Integrates with YouTube Music to automatically generate playlists and search/add tracks using browser session authentication.
- Supports secure API key and browser session storage in the system keyring plus environment-variable fallbacks.
- Prints generated playlist output (playlist name, selected platform, and songs by position) to stdout.

## Prerequisites
- Rust toolchain that supports edition `2024`.
- Cargo.
- Python 3.9+ with `uv` or `pip` installed.
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

SetlisteR resolves authentication credentials in this order:
1. Keyring entry
2. Environment variable fallback (`SETLIST_FM_API_KEY`, `YOUTUBE_HEADERS`, `YOUTUBE_OAUTH_JSON`)

### 1. Store setlist.fm API key in keyring
Run once with `--setlist-api-key`:
```bash
cargo run -- --artists "Radiohead" --service spotify --setlist-api-key "<YOUR_SETLIST_FM_API_KEY>"
```
Or export the environment variable:
```bash
export SETLIST_FM_API_KEY="<YOUR_SETLIST_FM_API_KEY>"
```

### 2. Store YouTube Music browser headers in keyring
To create playlists on YouTube Music, you must authorize your session by pasting your browser request headers:

1. Open a web browser tab and log into [music.youtube.com](https://music.youtube.com).
2. Open **Developer Tools** (`F12` or `Ctrl+Shift+I`) and select the **Network** tab.
3. Refresh the page or click a menu item (e.g. "Library") to trigger requests. Find an authenticated **POST** request (filtering by `/browse` is recommended).
4. Click on the request, select the **Headers** tab, scroll down to **Request Headers**, and copy the entire block of text.
5. Run SetlisteR with `-` as the value for `--youtube-headers` to read securely from standard input (stdin):
   ```bash
   cargo run -- --youtube-headers -
   ```
6. Paste the copied headers into the terminal and press **`Ctrl+D`** (on Linux/macOS) or **`Ctrl+Z` followed by `Enter`** (on Windows) to finish. The application will sanitize the headers and save the authenticated session JSON in your keyring.

Alternatively, you can export the raw headers as an environment variable (use single quotes `'` to wrap the value, as request headers contain double quotes `"`):
```bash
export YOUTUBE_HEADERS='<PASTE_YOUR_COPIED_HEADERS>'
```

## Usage

### Arguments
`cargo run -- --help` reports:

```text
Usage: SetlisteR [OPTIONS] --artists <ARTISTS> --service <SERVICE>

Options:
  -a, --artists <ARTISTS>
          Comma-separated list of artists to process (max 10, max length 100 per name)
  -p, --playlist-name <PLAYLIST_NAME>
          Custom playlist name (max length 100; auto-generated if omitted)
  -s, --service <SERVICE>
          Streaming service [possible values: spotify, youtube-music]
      --page-depth <PAGE_DEPTH>
          Number of setlist pages to fetch [default: 1]
      --setlist-api-key <SETLIST_API_KEY>
          Store the setlist.fm API key in the system keyring
      --youtube-headers <YOUTUBE_HEADERS>
          Store the YouTube browser authentication headers in the system keyring
  -h, --help
          Print help
```

### Examples

Generate a Spotify playlist (auto-generated name):
```bash
cargo run -- --artists "Radiohead" --service spotify
```

Generate a YouTube Music playlist using page depth 3 and a custom name:
```bash
cargo run -- --artists "Radiohead, Muse" --service youtube-music --page-depth 3 --playlist-name "Alternative Rock Mix"
```

## License
This project is licensed under the terms described in the [License](./LICENSE).

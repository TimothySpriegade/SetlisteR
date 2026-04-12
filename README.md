# SetlisteR
> SetlisteR is a Rust-based command-line tool that generates playlists based on artist setlists. By connecting to the setlist.fm API, it fetches recent setlist data for specified artists and can target different streaming services for playlist creation.

## Features
- Fetch setlist data from setlist.fm for multiple artists.
- Selectable streaming service support (currently supports Spotify and YouTube Music).
- Configurable pagination depth to fetch more extensive setlist histories.
- Environment variable-based configuration and system keyring support for secure API key management.

## Prerequisites
- **Rust**: Version 1.70 or higher (or a recent stable version supporting edition 2024).
- **Cargo**: Rust's package manager and build system.
- A valid [setlist.fm API key](https://api.setlist.fm/docs/1.0/index.html).

## Installation
1. Clone the repository to your local machine:
   ```bash
   git clone <repository-url>
   cd SetlisteR
   ```
2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

## Configuration
SetlisteR requires a setlist.fm API key to function. You can configure this key securely in your system keyring or via an environment variable.

### System Keyring (Recommended)
You can store your API key securely in your operating system's native keyring (Keychain on macOS, Credential Manager on Windows, Secret Service on Linux) by running:
```bash
cargo run -- --artists "Artist" --service spotify --setlist-api-key "<your super private setlist.fm API key here>"
```
Once stored, SetlisteR will automatically retrieve it for future runs. This means you technically only need to set the API key once, and subsequent commands do not need to include the `--setlist-api-key` flag.

### Environment Variable
Alternatively, you can provide the API key via the `SETLIST_FM_API_KEY` system environment variable. This is useful for CI/CD environments or if you prefer not to use the system keyring.

For example, in a Unix-like shell:
```bash
export SETLIST_FM_API_KEY="<your super private setlist.fm API key here>"
cargo run -- --artists "Radiohead" --service spotify
```

## Usage

### Arguments
SetlisteR accepts the following command-line arguments:

- `-a, --artists <ARTISTS>`: Artists to generate a setlist playlist for, separated by commas.
- `-p, --playlist-name <PLAYLIST_NAME>`: The name of the playlist to create (optional).
- `-s, --service <SERVICE>`: Streaming service to use. Accepted values: `spotify`, `youtube_music`.
- `--page-depth <PAGE_DEPTH>`: How many pages of setlist data to fetch from the setlist.fm API. Defaults to `1`.
- `--setlist-api-key <SETLIST_API_KEY>`: Store the setlist.fm API key in the system keyring.

### Examples
Fetch setlist data for a single artist and target Spotify:
```bash
cargo run -- --artists "Radiohead" --service spotify
```

Fetch setlist data for multiple artists, checking 3 pages deep, and target YouTube Music:
```bash
cargo run -- --artists "Radiohead, Muse" --service youtube_music --page-depth 3
```

Store your API key in the system keyring while fetching an artist (you only need to provide the flag once):
```bash
cargo run -- --artists "Radiohead" --service spotify --setlist-api-key "<YOUR_API_KEY>"
```

Then, you can run subsequent commands without needing the API key flag:
```bash
cargo run -- --artists "Radiohead" --service spotify
```

## Contributing
We welcome contributions to SetlisteR! Please see our [Contributing Guidelines](./CONTRIBUTING.md) for more details on how to get started, our development workflow, and testing instructions.

## License
This project is licensed under the terms described in the [License](./LICENSE).

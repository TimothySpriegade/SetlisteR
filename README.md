# SetlisteR
> SetlisteR is a Rust-based command-line tool that generates playlists based on artist setlists. By connecting to the setlist.fm API, it fetches recent setlist data for specified artists and can target different streaming services for playlist creation.

## Features
- Fetch setlist data from setlist.fm for multiple artists.
- Selectable streaming service support (currently supports Spotify and YouTube Music).
- Configurable pagination depth to fetch more extensive setlist histories.
- Environment variable-based configuration for secure API key management.

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
SetlisteR requires a setlist.fm API key to function.

1. Copy the provided environment template file:
   ```bash
   cp .env.template .env
   ```
2. Open the `.env` file and replace the placeholder with your actual API key:
   ```env
   SETLIST_FM_API_KEY=<your super private setlist.fm API key here>
   ```

## Usage

### Arguments
SetlisteR accepts the following command-line arguments:

- `-a, --artists <ARTISTS>`: Artists to generate a setlist playlist for, separated by commas.
- `-p, --playlist-name <PLAYLIST_NAME>`: The name of the playlist to create (optional).
- `-s, --service <SERVICE>`: Streaming service to use. Accepted values: `spotify`, `youtube_music`.
- `--page-depth <PAGE_DEPTH>`: How many pages of setlist data to fetch from the setlist.fm API. Defaults to `1`.

### Examples
Fetch setlist data for a single artist and target Spotify:
```bash
cargo run -- --artists "Radiohead" --service spotify
```

Fetch setlist data for multiple artists, checking 3 pages deep, and target YouTube Music:
```bash
cargo run -- --artists "Radiohead, Muse" --service youtube_music --page-depth 3
```

## Contributing
We welcome contributions to SetlisteR! Please see our [Contributing Guidelines](./CONTRIBUTING.md) for more details on how to get started, our development workflow, and testing instructions.

## License
This project is licensed under the terms described in the [License](./LICENSE.md).

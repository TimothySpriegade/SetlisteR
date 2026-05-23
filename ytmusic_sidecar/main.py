import argparse

from auth import login
from playlist import create_playlist


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="YouTube Music API Sidecar")
    subparsers = parser.add_subparsers(dest="command", required=True)

    login_parser = subparsers.add_parser("login", help="Authenticate with YouTube Music")
    login_parser.add_argument("--headers", required=True, help="Raw browser headers")
    login_parser.add_argument("--output-file", required=True, help="Path to write the auth JSON")

    subparsers.add_parser(
        "create-playlist",
        help="Create a playlist from JSON piped on stdin",
    )

    return parser


def main() -> None:
    args = _build_parser().parse_args()

    if args.command == "login":
        login(args.headers, args.output_file)
    elif args.command == "create-playlist":
        create_playlist()


if __name__ == "__main__":
    main()

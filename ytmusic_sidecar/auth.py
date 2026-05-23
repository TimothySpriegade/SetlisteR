import sys
import json

from ytmusicapi import setup


_INVALID_KEY_CHARS = frozenset("/? ")


def _is_valid_header_key(key: str) -> bool:
    return bool(key.strip()) and not any(c in key for c in _INVALID_KEY_CHARS)


def login(headers_raw: str, output_file: str) -> None:
    try:
        setup(filepath=output_file, headers_raw=headers_raw)

        with open(output_file) as f:
            headers: dict = json.load(f)

        cleaned = {k: v for k, v in headers.items() if _is_valid_header_key(k)}

        with open(output_file, "w") as f:
            json.dump(cleaned, f, indent=4)
    except Exception as e:
        print(f"Failed to set up browser authentication: {e}", file=sys.stderr)
        sys.exit(1)

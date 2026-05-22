import os
import sys
import tempfile
from ytmusicapi.auth.oauth import setup_oauth

def login(client_id, client_secret):
    with tempfile.TemporaryDirectory() as temp_dir:
        oauth_file = os.path.join(temp_dir, "oauth.json")
        try:
            setup_oauth(
                client_id=client_id,
                client_secret=client_secret,
                filepath=oauth_file,
                open_browser=False
            )
            
            with open(oauth_file, "r") as f:
                oauth_data = f.read()
            
            print("\n---YT_OAUTH_JSON_START---")
            print(oauth_data)
            print("---YT_OAUTH_JSON_END---")
            
        except Exception as e:
            print(f"Error during OAuth setup: {e}", file=sys.stderr)
            sys.exit(1)

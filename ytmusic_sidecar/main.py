import argparse
from auth import login
from playlist import create_playlist

def main():
    parser = argparse.ArgumentParser(description="YouTube Music API Sidecar")
    subparsers = parser.add_subparsers(dest="command", required=True)
    
    login_parser = subparsers.add_parser("login")
    login_parser.add_argument("--client-id", required=True)
    login_parser.add_argument("--client-secret", required=True)
    
    subparsers.add_parser("create-playlist")
    
    args = parser.parse_args()
    
    if args.command == "login":
        login(args.client_id, args.client_secret)
    elif args.command == "create-playlist":
        create_playlist()

if __name__ == "__main__":
    main()

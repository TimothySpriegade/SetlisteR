#!/usr/bin/env bash
# =============================================================================
# create_issues.sh
#
# Creates the initial set of GitHub issues for the SetlisteR repository
# using the GitHub CLI (gh).
#
# Usage:
#   chmod +x .github/scripts/create_issues.sh
#   ./.github/scripts/create_issues.sh
#
# Prerequisites:
#   - GitHub CLI (gh) must be installed and authenticated (`gh auth login`)
#   - Run from the root of the repository, or set REPO below explicitly.
# =============================================================================

set -euo pipefail

# ---------------------------------------------------------------------------
# Helper: create an issue using the feature_request template format
# ---------------------------------------------------------------------------
create_feature_issue() {
  local title="$1"
  local problem_statement="$2"
  local proposed_solution="$3"
  local alternatives="$4"
  local streaming_service="$5"
  local additional_context="$6"
  shift 6
  local labels=("$@")

  local body
  body="### Problem Statement

${problem_statement}

### Proposed Solution

${proposed_solution}"

  if [[ -n "$alternatives" ]]; then
    body="${body}

### Alternatives Considered

${alternatives}"
  fi

  if [[ -n "$streaming_service" ]]; then
    body="${body}

### Relevant Streaming Service

${streaming_service}"
  fi

  if [[ -n "$additional_context" ]]; then
    body="${body}

### Additional Context

${additional_context}"
  fi

  local label_args=()
  for label in "${labels[@]}"; do
    label_args+=("--label" "$label")
  done

  if gh issue create \
      --title "$title" \
      --body "$body" \
      "${label_args[@]}" 2>/dev/null; then
    echo "  ✔  '$title'"
  else
    echo "  ✘  '$title' (failed)"
  fi
}

echo "Creating issues for $(gh repo view --json nameWithOwner -q .nameWithOwner) ..."
echo ""

# ---------------------------------------------------------------------------
# Phase 1: Setlist.fm Ingestion & Analysis
# ---------------------------------------------------------------------------
echo "Phase 1: Setlist.fm Ingestion & Analysis"

create_feature_issue \
  "[Feature]: Setlist.fm API Client" \
  "There is currently no way to retrieve setlist data from Setlist.fm, which is required before any statistical analysis or playlist generation can take place." \
  "Implement an API client module that interfaces with the Setlist.fm REST API. The client should:
- Authenticate using a Setlist.fm API key (read from environment / config).
- Expose a function to fetch recent setlists for a given artist (by name or MBID).
- Handle pagination so all available setlists are retrievable.
- Return raw API responses in a structured format ready for downstream parsing." \
  "Web scraping Setlist.fm directly was considered but is fragile and violates the terms of service." \
  "Both / General" \
  "Setlist.fm API documentation: https://api.setlist.fm/docs/1.0/index.html
Credentials are stored as environment variables (see \`.env.template\`)." \
  "enhancement"

create_feature_issue \
  "[Feature]: Setlist.fm Data Sanitization" \
  "The raw JSON returned by the Setlist.fm API contains a large amount of metadata that is irrelevant to statistical analysis. Operating on the raw payload adds noise and increases memory pressure." \
  "Implement a data-sanitization layer that parses the raw Setlist.fm API response into a minimal, strongly-typed struct optimised purely for algorithmic and statistical processing. The struct should capture only:
- Artist identifier.
- Setlist date.
- Ordered list of song titles (and optionally their position in the set).
All extraneous fields (venue details, tour info, version metadata, etc.) must be discarded at parse time." \
  "A single all-purpose struct that preserves all fields was considered, but was rejected because it couples the analysis engine to the API schema and harms performance." \
  "Both / General" \
  "This module sits between the API client (Setlist.fm API Client issue) and the statistical analysis engine, and its output struct is the sole input type consumed by the analysis engine." \
  "enhancement"

create_feature_issue \
  "[Feature]: Statistical Analysis Engine" \
  "Without a way to statistically analyse historical setlist data, SetlisteR cannot predict which songs an artist is most likely to play at an upcoming show." \
  "Implement a statistical analysis engine that consumes the sanitized setlist structs and produces a ranked, predicted setlist for a given artist. Requirements:
- Calculate song-play frequency and recency-weighted probability scores.
- Produce a predicted setlist as an ordered list of song titles.
- Support parallel execution so that predictions for multiple artists can be computed simultaneously (e.g., via Rust's \`rayon\` crate or async tasks).
- Expose a clean public API that the CLI and future integrations can call." \
  "A purely sequential implementation was considered but was ruled out because querying several artists at once would be unacceptably slow." \
  "Both / General" \
  "Parallelism strategy (thread pool vs. async) should be decided during implementation based on I/O vs. CPU characteristics of the workload." \
  "enhancement"

echo ""

# ---------------------------------------------------------------------------
# Phase 2: Streaming Platform Generation
# ---------------------------------------------------------------------------
echo "Phase 2: Streaming Platform Generation"

create_feature_issue \
  "[Feature]: Spotify Integration" \
  "Users who use Spotify currently have no way to automatically generate a playlist of the predicted setlist on their account." \
  "Implement a Spotify integration module that:
- Authenticates with the Spotify Web API using the OAuth 2.0 Authorization Code flow (credentials stored in environment variables).
- Searches for each predicted song by title and artist to resolve the Spotify track URI.
- Creates a new playlist in the authenticated user's Spotify account (or updates an existing one with the same name).
- Adds the resolved tracks to the playlist in predicted setlist order.
The module must be callable from the existing CLI (\`service: spotify\` argument) and accept the output of the statistical analysis engine as input." \
  "Using a pre-built Spotify SDK crate was considered; the decision of whether to use one (e.g., \`rspotify\`) or call the REST API directly should be made during implementation." \
  "Spotify" \
  "Spotify Web API reference: https://developer.spotify.com/documentation/web-api
Required OAuth scopes: \`playlist-modify-public\` and/or \`playlist-modify-private\`." \
  "enhancement" "spotify"

create_feature_issue \
  "[Feature]: YouTube Music Integration" \
  "Users who use YouTube Music currently have no way to automatically generate a playlist of the predicted setlist on their account." \
  "Implement a YouTube Music integration module that:
- Authenticates with the YouTube Data API v3 using OAuth 2.0 (credentials stored in environment variables).
- Searches for each predicted song by title and artist to resolve the YouTube video ID.
- Creates a new playlist in the authenticated user's YouTube Music account (or updates an existing one with the same name).
- Inserts the resolved videos into the playlist in predicted setlist order.
The module must be callable from the existing CLI (\`service: youtube_music\` argument) and accept the output of the statistical analysis engine as input." \
  "Using an unofficial YouTube Music API was considered but was rejected due to stability and ToS concerns; the official YouTube Data API v3 is preferred." \
  "YouTube Music" \
  "YouTube Data API v3 reference: https://developers.google.com/youtube/v3
Required OAuth scopes: \`https://www.googleapis.com/auth/youtube\`." \
  "enhancement" "youtube music"

echo ""
echo "Done! All issues have been processed."

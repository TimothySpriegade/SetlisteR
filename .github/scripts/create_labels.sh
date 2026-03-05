#!/usr/bin/env bash
# =============================================================================
# create_labels.sh
#
# Creates the standard set of GitHub issue labels for the SetlisteR repository
# using the GitHub CLI (gh).
#
# Usage:
#   chmod +x .github/scripts/create_labels.sh
#   ./.github/scripts/create_labels.sh
#
# Prerequisites:
#   - GitHub CLI (gh) must be installed and authenticated (`gh auth login`)
#   - Run from the root of the repository, or set REPO below explicitly.
# =============================================================================

set -euo pipefail

# ---------------------------------------------------------------------------
# Helper: create or update a label (update if it already exists)
# ---------------------------------------------------------------------------
create_label() {
  local name="$1"
  local color="$2"
  local description="$3"

  if gh label create "$name" \
      --color "$color" \
      --description "$description" \
      --force 2>/dev/null; then
    echo "  ✔  '$name'"
  else
    echo "  ✘  '$name' (failed)"
  fi
}

echo "Creating labels for $(gh repo view --json nameWithOwner -q .nameWithOwner) ..."
echo ""

# ---------------------------------------------------------------------------
# Standard labels
# ---------------------------------------------------------------------------
create_label "bug"             "d73a4a" "Something isn't working as expected"
create_label "enhancement"     "a2eeef" "New feature or improvement request"
create_label "documentation"   "0075ca" "Improvements or additions to documentation"
create_label "good first issue" "7057ff" "Good for newcomers to the project"
create_label "help wanted"     "008672" "Extra attention or community help is needed"
create_label "duplicate"       "cfcfa3" "This issue or pull request already exists"
create_label "invalid"         "e4e669" "This doesn't seem right or is off-topic"
create_label "wontfix"         "ffffff" "This will not be worked on"
create_label "priority: high"  "B60205" "Must be addressed urgently"
create_label "priority: low"   "F9D0C4" "Can be addressed when time allows"

# ---------------------------------------------------------------------------
# SetlisteR-specific labels
# ---------------------------------------------------------------------------
create_label "spotify"         "1DB954" "Related to the Spotify integration"
create_label "youtube music"   "FF0000" "Related to the YouTube Music integration"
create_label "cli"             "5319E7" "Related to command-line interface / argument handling"
create_label "dependencies"    "0366d6" "Pull requests that update a dependency"

echo ""
echo "Done! All labels have been processed."

#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "Usage: $0 <tag> [output-file]"
  echo "Example: $0 v0.8.1 RELEASE_NOTES.md"
  exit 2
fi

tag="$1"
output_file="${2:-RELEASE_NOTES.md}"
changelog_file="${CHANGELOG_FILE:-CHANGELOG.md}"

if [[ ! -f "$changelog_file" ]]; then
  echo "Changelog file not found: $changelog_file"
  exit 1
fi

awk -v tag="$tag" '
  $0 == "## [" tag "]" || index($0, "## [" tag "] - ") == 1 {
    found = 1
    print
    next
  }
  found && /^## \[/ {
    exit
  }
  found {
    print
  }
  END {
    if (!found) {
      exit 1
    }
  }
' "$changelog_file" > "$output_file"

if [[ ! -s "$output_file" ]]; then
  echo "No release notes generated for $tag"
  exit 1
fi

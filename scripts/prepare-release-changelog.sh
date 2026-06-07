#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <version|tag>"
  echo "Example: $0 0.8.1"
  exit 2
fi

input="$1"
if [[ "$input" == v* ]]; then
  tag="$input"
  version="${input#v}"
else
  version="$input"
  tag="v$input"
fi

if ! [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$ ]]; then
  echo "Invalid release version: $version"
  echo "Use a semantic version like 0.8.1"
  exit 1
fi

changelog_file="${CHANGELOG_FILE:-CHANGELOG.md}"

if [[ ! -f "$changelog_file" ]]; then
  echo "Changelog file not found: $changelog_file"
  exit 1
fi

if ! grep -q '^## \[Unreleased\]' "$changelog_file"; then
  echo "Missing Unreleased section in $changelog_file"
  exit 1
fi

release_body="$(mktemp)"
header_file="$(mktemp)"
tail_file="$(mktemp)"
next_file="$(mktemp)"
trap 'rm -f "$release_body" "$header_file" "$tail_file" "$next_file"' EXIT

git-cliff --config cliff.toml --tag "$tag" --unreleased --strip header > "$release_body"

if [[ ! -s "$release_body" ]]; then
  echo "git-cliff generated an empty changelog for $tag"
  exit 1
fi

awk '/^## \[Unreleased\]/{exit} {print}' "$changelog_file" > "$header_file"

awk '
  /^## \[Unreleased\]/ { in_unreleased = 1; next }
  in_unreleased && /^## \[/ { in_unreleased = 0; tail = 1; print; next }
  tail { print }
' "$changelog_file" > "$tail_file"

{
  cat "$header_file"
  printf '## [Unreleased]\n\n'
  cat "$release_body"
  printf '\n'
  cat "$tail_file"
} > "$next_file"

mv "$next_file" "$changelog_file"

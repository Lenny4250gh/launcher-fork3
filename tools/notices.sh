#!/usr/bin/env bash
# Writes the licence file of a release: the licences of every third-party crate in the launcher
# and in the net service and share worker it carries (about.toml here and in anyplaynet/network),
# then Cisco's licence for the OpenH264 binary the launcher downloads on Linux.
#
#   notices.sh <output file>
#
# Run it from a launcher checkout with anyplaynet/network beside it. Build the pinned
# cargo-about from source with its locked dependencies; never execute a downloaded prebuilt
# helper. CARGO_ABOUT may name an already source-built native cargo-about binary.
set -euo pipefail

CARGO_ABOUT_VERSION=0.9.2
mkdir -p "$(dirname "${1:?output file}")"
out=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/.." && pwd)
network=$(cd "$root/../network" && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

about=${CARGO_ABOUT:-}
if [ -z "$about" ]; then
  cargo install --locked --features cli --version "$CARGO_ABOUT_VERSION" --root "$work/about" cargo-about
  about="$work/about/bin/cargo-about"
fi

# --fail stops on a crate whose licence cannot be read: a release never ships an incomplete file.
"$about" generate --format json --locked --fail --workspace \
  -c "$root/about.toml" -m "$root/Cargo.toml" -o "$work/launcher.json"
"$about" generate --format json --locked --fail \
  -c "$network/about.toml" -m "$network/crates/net/Cargo.toml" -o "$work/net.json"
"$about" generate --format json --locked --fail \
  -c "$network/about.toml" -m "$network/crates/share-worker/Cargo.toml" -o "$work/share.json"
mkdir -p "$(dirname "$out")"
node "$here/notices.mjs" "$out" "$work/launcher.json" "$work/net.json" "$work/share.json"
# C code built into the launcher from source, which cargo-about does not see: LibAtrac9 decodes the
# games' own music (crates/atrac9).
{
  printf '\n%s\n%s\n\n' "LibAtrac9 by Alex Barney (C source in crates/atrac9/libatrac9, built into the launcher" \
    "to play a game's own music on the home screen):"
  cat "$root/crates/atrac9/libatrac9/LICENSE"
} >> "$out"
# Not part of the release: Cisco's own binary, downloaded to the player's PC when they turn the
# codec on. Cisco's licence asks for its full text where licences are shown.
{
  printf '\n%s\n%s\n\n' "Cisco's OpenH264 binary (Linux, downloaded from Cisco when the player turns" \
    "Share Screen Video Codec on in Settings, Advanced):"
  cat "$here/openh264/BINARY_LICENSE.txt"
} >> "$out"

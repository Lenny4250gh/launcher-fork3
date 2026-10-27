#!/usr/bin/env bash
# Packs a built Linux launcher as a Flatpak bundle.
#
#   flatpak.sh <version> <launcher binary> <licence file> <output folder>
#
# Same arguments as package.sh. Nothing is compiled: the manifest installs the given binary.
# Needs flatpak and flatpak-builder; the runtime and SDK come from Flathub (the only download).
# Writes AnyPlay-Launcher-<version>-x86_64.flatpak into the output folder.
set -euo pipefail

version=${1:?version}
binary=$(realpath "${2:?launcher binary}")
notices=$(realpath "${3:?licence file}")
out=$(mkdir -p "${4:?output folder}" && realpath "$4")
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
src="$root/packaging/flatpak"
app=com.anyplaynet.Launcher
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

# The manifest builds from ./stage beside it.
stage="$work/stage"
mkdir -p "$stage"
install -m 755 "$binary" "$stage/anyplay-launcher"
install -m 644 "$here/anyplay-launcher.desktop" "$here/anyplay-launcher.png" "$root/LICENSE" "$stage/"
install -m 644 "$notices" "$stage/THIRD-PARTY-LICENSES.txt"
sed -e "s/@VERSION@/$version/" -e "s/@DATE@/$(date -u +%Y-%m-%d)/" \
  "$src/$app.metainfo.xml" > "$stage/$app.metainfo.xml"
cp "$src/$app.yml" "$work/$app.yml"

if command -v appstreamcli >/dev/null; then
  appstreamcli validate --no-net "$stage/$app.metainfo.xml"
fi

flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak-builder --user --force-clean --install-deps-from=flathub \
  --repo="$work/repo" --state-dir="$work/state" "$work/build" "$work/$app.yml"
flatpak build-bundle "$work/repo" "$out/AnyPlay-Launcher-$version-x86_64.flatpak" "$app"
ls -l "$out"

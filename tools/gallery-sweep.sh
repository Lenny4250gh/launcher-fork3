#!/usr/bin/env bash
# Renders every gallery scene headless and fails on a panic, a hang or a missing picture.
#
#   tools/gallery-sweep.sh <launcher-binary> [jobs] [seconds-per-scene] [shard/shards]
#
# `shard/shards` (for example 3/8) renders only every 8th scene of the sorted list, starting at the
# 3rd: the CI job `scenes` runs 8 of these side by side, each on a runner of its own, from one debug
# build. Every scene falls into exactly one shard; without the argument all scenes are rendered.
#
# Meant for a DEBUG build on Linux under xvfb with a software Vulkan device (see ci.yml): egui's
# debug lock panics after 10 s on a deadlock (0.2.8 froze on `response.has_focus()` inside
# `ctx.data_mut`), where a release build only hangs. Scene names come from the source, so a scene
# another change adds is swept without touching this script. One process per scene: a crash or a
# hang names the scene and cannot hide the rest.
set -uo pipefail

BIN=$(realpath "${1:?usage: gallery-sweep.sh <binary> [jobs] [seconds]}")
JOBS=${2:-4}
LIMIT=${3:-60}
SHARD=${4:-1/1}
if [[ $SHARD =~ ^([0-9]+)/([0-9]+)$ ]] && (( BASH_REMATCH[1] >= 1 && BASH_REMATCH[1] <= BASH_REMATCH[2] )); then
  SHARD_INDEX=${BASH_REMATCH[1]}; SHARD_COUNT=${BASH_REMATCH[2]}
else
  echo "shard must look like 3/8 (index from 1, up to the count), got '$SHARD'" >&2; exit 2
fi
ROOT=$(cd "$(dirname "$0")/.." && pwd)
SRC=$ROOT/crates/anyps5-launcher/src
OUT=${SWEEP_OUT:-$(mktemp -d)}
mkdir -p "$OUT"

scenes() {
  # `"name" => Scene::…` and `"name" | "alias" => …` arms of the home scenes and the panel pages.
  grep -hoE '^\s+"[a-z0-9-]+"( \| "[a-z0-9-]+")* => (Scene|ScreenPage)::' "$SRC/app/preview.rs" "$SRC/screens/gallery.rs" \
    | grep -oE '"[a-z0-9-]+"' | tr -d '"'
  # `together-<blocker>` names come from the BLOCKERS list.
  sed -n '/^const BLOCKERS/,/^];/p' "$SRC/screens/gallery_together.rs" | grep -oE '"[a-z0-9-]+"' | tr -d '"' | sed 's/^/together-/'
  echo together-offer; echo together-offer-invitation
  # The foundation's own pages.
  printf '%s\n' components panels states type artwork themes cosmic aurora sunset
}

one() {
  scene=$1
  png=$OUT/$scene.png
  log=$OUT/$scene.log
  # A fresh data folder per scene: nothing of the runner's (or a previous scene's) state leaks in.
  home=$(mktemp -d)
  env -u WAYLAND_DISPLAY HOME="$home" XDG_DATA_HOME="$home/data" XDG_CONFIG_HOME="$home/config" \
    VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json \
    timeout -k 5 "$LIMIT" \
    "$BIN" --gallery="$scene" --screenshot="$png" --size=1280x720 --frames=12 >"$log" 2>&1
  code=$?
  rm -rf "$home"
  if [ $code -eq 124 ] || [ $code -eq 137 ]; then
    echo "HANG   $scene (no picture after ${LIMIT}s)"; return 1
  elif [ $code -ne 0 ]; then
    echo "CRASH  $scene (exit $code)"; return 1
  elif [ ! -s "$png" ]; then
    echo "NOPIC  $scene (exited 0 without a picture)"; return 1
  elif grep -qE "panicked at|deadlock" "$log"; then
    echo "PANIC  $scene"; return 1
  fi
  return 0
}
# One X server of its own for all workers, whatever DISPLAY the machine has (several xvfb-run -a at
# once race for display numbers, and a desktop X server dies under four windows at a time).
DISPLAY_NUMBER=$((100 + $$ % 800))
Xvfb ":$DISPLAY_NUMBER" -screen 0 1920x1080x24 -nolisten tcp >/dev/null 2>&1 &
XVFB=$!
trap 'kill $XVFB 2>/dev/null' EXIT
export DISPLAY=":$DISPLAY_NUMBER"
sleep 2
export -f one
export BIN OUT LIMIT

LIST=$OUT/scenes.txt
scenes | sort -u | awk -v i="$SHARD_INDEX" -v n="$SHARD_COUNT" 'NR % n == i % n' > "$LIST"
echo "sweeping $(wc -l < "$LIST") scenes (shard $SHARD) with $JOBS jobs, ${LIMIT}s each"
FAILED=$OUT/failed.txt
: > "$FAILED"
xargs -a "$LIST" -P "$JOBS" -I{} bash -c 'one {} >> "'"$FAILED"'" || true'

if [ -s "$FAILED" ]; then
  sort "$FAILED"
  for scene in $(awk '{print $2}' "$FAILED" | sort -u); do
    echo "---- $scene ----"; grep -vE '^\s*$' "$OUT/$scene.log" | tail -25 | cut -c1-300
  done
  echo "$(wc -l < "$FAILED") of $(wc -l < "$LIST") scenes failed"
  exit 1
fi
echo "all $(wc -l < "$LIST") scenes rendered"

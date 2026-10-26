#!/usr/bin/env bash
# Installs Debian/Ubuntu packages on a CI runner without letting a slow mirror stall the job.
#
#   tools/apt-install.sh <package>...
#
# Packages the runner image already has are skipped (no `apt-get update` at all when nothing is
# missing). Each attempt has a time limit and apt retries a broken download itself: a mirror that
# hangs cost a scenes shard 12+ minutes (one `apt-get install` that never returned), where a retry
# after 2 minutes usually gets a good mirror.
set -uo pipefail
missing=()
for package in "$@"; do
  dpkg -s "$package" >/dev/null 2>&1 || missing+=("$package")
done
if [ ${#missing[@]} -eq 0 ]; then
  echo "all $# packages are already installed"
  exit 0
fi
echo "installing ${#missing[@]} of $# packages: ${missing[*]}"
opts=(-o Acquire::Retries=3 -o Acquire::http::Timeout=20 -o Acquire::https::Timeout=20)
for attempt in 1 2 3; do
  if timeout -k 10 150 sudo apt-get "${opts[@]}" update \
    && timeout -k 10 240 sudo apt-get "${opts[@]}" install -y --no-install-recommends "${missing[@]}"; then
    exit 0
  fi
  echo "apt attempt $attempt failed or timed out, trying again"
done
exit 1

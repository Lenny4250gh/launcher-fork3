#!/usr/bin/env bash
# Installs Debian/Ubuntu packages on a CI runner without letting a stalled mirror hold the job.
#
#   tools/apt-install.sh <package>...
#
# Packages the runner image already has are skipped (no `apt-get update` at all when nothing is
# missing). Each attempt has a short time limit (a package of 17 MB normally takes a second) and
# the later attempts leave the runner's default mirror, azure.archive.ubuntu.com, for Canonical's
# and then kernel.org's:
# a scenes shard sat 12 minutes on one 17 MB download that stalled on the Azure mirror, three
# times in a row, while the same download from another mirror takes seconds.
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
opts=(-o Acquire::Retries=2 -o Acquire::http::Timeout=15 -o Acquire::https::Timeout=15)
# The runner's default mirror first, then two others. Each attempt rewrites whichever of them the
# sources name now, so the order of the list is the order of the attempts.
mirrors=(http://archive.ubuntu.com/ubuntu/ http://mirrors.edge.kernel.org/ubuntu/)
known='(mirror[+]file:/etc/apt/apt-mirrors[.]txt|https?://(azure[.]archive[.]ubuntu[.]com|archive[.]ubuntu[.]com|mirrors[.]edge[.]kernel[.]org)/ubuntu/?)'
for attempt in 1 2 3; do
  if [ "$attempt" -gt 1 ]; then
    mirror=${mirrors[$((attempt - 2))]}
    echo "apt attempt $((attempt - 1)) failed or timed out: trying $mirror"
    sudo sed -i -E "s#$known#$mirror#" /etc/apt/sources.list /etc/apt/sources.list.d/*.sources 2>/dev/null || true
  fi
  if timeout -k 5 60 sudo apt-get "${opts[@]}" update \
    && timeout -k 5 90 sudo apt-get "${opts[@]}" install -y --no-install-recommends "${missing[@]}"; then
    exit 0
  fi
done
echo "apt did not install ${missing[*]}" >&2
exit 1

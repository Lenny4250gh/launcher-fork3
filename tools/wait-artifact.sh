#!/usr/bin/env bash
# Waits until a workflow artifact of THIS run has been uploaded by another job.
#
#   tools/wait-artifact.sh <artifact-name> [minutes]
#
# The release builds start at once and only need the licence file (`dist-notices`) for their last
# step; this waits for it there instead of holding the whole job back. Needs GH_TOKEN and an
# `actions: read` token. Fails after the limit, so a failed licence job does not hang the run.
set -euo pipefail
name=${1:?usage: wait-artifact.sh <artifact-name> [minutes]}
limit=$(( ${2:-20} * 60 ))
start=$(date +%s)
while :; do
  count=$(gh api "repos/$GITHUB_REPOSITORY/actions/runs/$GITHUB_RUN_ID/artifacts?name=$name" --jq '.artifacts | map(select(.expired | not)) | length')
  if [ "$count" -ge 1 ]; then
    echo "artifact $name is there after $(( $(date +%s) - start )) s"
    exit 0
  fi
  if [ $(( $(date +%s) - start )) -gt "$limit" ]; then
    echo "::error::artifact $name did not appear within $((limit / 60)) minutes (did the job that makes it fail?)"
    exit 1
  fi
  sleep 10
done

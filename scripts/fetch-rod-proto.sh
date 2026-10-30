#!/usr/bin/env bash
# Fetches the platform's wire contract (rod.proto) into protos/, the probe
# site build.rs reads for a standalone checkout of this repository. The
# proto stays the platform repository's contract; this copy is a build
# input, never tracked.
#
# Usage: scripts/fetch-rod-proto.sh [ref]   (default: main)
set -euo pipefail

ref="${1:-main}"
root="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "$root/protos"

url="https://raw.githubusercontent.com/rodframework/Rod/${ref}/src/teamserver/Rod.Protocol/protos/rod.proto"

# Retries with backoff: raw.githubusercontent has bad minutes.
for attempt in 1 2 3 4 5; do
    if curl -fsSL "$url" -o "$root/protos/rod.proto"; then
        echo "fetched rod.proto at ${ref}"
        exit 0
    fi
    sleep "$((attempt * attempt))"
done
echo "could not fetch rod.proto at ${ref} from ${url}" >&2
exit 1

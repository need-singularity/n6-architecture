#!/usr/bin/env bash
# Quick discovery run — engine only, <5 seconds
exec "$(dirname "$0")/discovery_pipeline.sh" --quick

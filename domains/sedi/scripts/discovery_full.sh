#!/usr/bin/env bash
# Full discovery pipeline — engine + miner + grade + sync
exec "$(dirname "$0")/discovery_pipeline.sh" --full

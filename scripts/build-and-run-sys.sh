#!/usr/bin/sh
set -e

# Run from parent folder

./liquislime-plugins/build-plugins.sh
./scripts/run-sys.sh

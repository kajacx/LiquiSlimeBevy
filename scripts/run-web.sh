#!/usr/bin/sh
set -e

# Run from parent folder

./scripts/build-plugins.sh
./scripts/start-web.sh

echo "Restarting webserver"
cd liquislime-web/liquislime-webserver
docker-compose down && docker-compose up -d

echo "All done, play the game at http://127.0.0.1:8088/"

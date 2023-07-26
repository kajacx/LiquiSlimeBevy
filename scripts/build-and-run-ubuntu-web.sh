#!/usr/bin/sh
set -e

# Run from the parent folder

./liquislime-plugins/build-plugins.sh
./scripts/run-web.sh --release

echo "Restarting webserver"
cd liquislime-web/liquislime-webserver
sudo docker-compose down && sudo docker-compose up -d

echo "All done, play the game at http://liquislime.com:8088/"

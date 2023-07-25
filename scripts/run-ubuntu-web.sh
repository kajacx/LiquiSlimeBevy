#!/usr/bin/sh
set -e

# Run from the parent folder

sh scripts/build-plugins.sh
sh scripts/start-web.sh --release

echo "Restarting webserver"
cd liquislime-web/liquislime-webserver
sudo docker-compose down && sudo docker-compose up -d
cd ../..

echo "All done, play the game at http://liquislime.com:8088/"

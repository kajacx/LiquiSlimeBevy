#!/usr/bin/bash

docker build -t liquislime-web . && \
docker run --name liquislime-web-test liquislime-web

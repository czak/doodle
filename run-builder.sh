#!/bin/sh
docker run --rm -it \
  --workdir /usr/src/scribble \
  --volume $PWD/scribble:/usr/src/scribble \
  rust:1.42.0-alpine3.11 \
  sh

#!/bin/sh
docker run --rm -it \
  -e GIT_USER_EMAIL \
  -e GIT_USER_NAME \
  -e REPO_URL \
  -e BRANCH_NAME \
  -e USERNAME \
  -v $HOME/.ssh/id_rsa:/root/.ssh/id_rsa \
  doodle-go \
  sh

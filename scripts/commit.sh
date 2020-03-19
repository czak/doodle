#!/bin/sh
git config --global user.email $GIT_USER_EMAIL
git config --global user.name $GIT_USER_NAME
git clone $REPO_URL --branch $BRANCH_NAME --single-branch /checkout

weekday=$(date +'%u')
for i in `seq 1 $weekday`; do
  scribble index.html
  git commit -am "I accidentally a line #$i"
  git push
  sleep 1
done

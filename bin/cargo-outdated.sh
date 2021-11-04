#!/bin/sh

echo > z.outdated.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    cargo outdated
  })
  echo
done 2>&1 | tee -a z.outdated.log


#!/bin/sh

echo > z.update.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    cargo update
  })
  echo
done 2>&1 | tee -a z.update.log


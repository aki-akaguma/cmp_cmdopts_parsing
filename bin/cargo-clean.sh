#!/bin/sh

echo > z.clean.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    cargo clean
  })
  echo
done 2>&1 | tee -a z.clean.log


#!/bin/sh

echo > z.build.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    cargo clippy
  })
  echo
done 2>&1 | tee -a z.clippy.log


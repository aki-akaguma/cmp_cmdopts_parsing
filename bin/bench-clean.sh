#!/bin/sh

echo > z.bench-clean.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    [ -d "$nm/target/criterion" ] && rm -fr "$nm/target/criterion"
  })
  echo
done 2>&1 | tee -a z.bench-clean.log


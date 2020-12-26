#!/bin/sh

echo > z.bench-release-s.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    env -u CARGO_PROFILE_RELEASE_LTO -u CARGO_PROFILE_RELEASE_CODEGEN_UNITS \
      CARGO_PROFILE_RELEASE_OPT_LEVEL="s" \
      taskset --cpu-list 2 cargo bench --bench bench-one -- --noplot
  })
  echo
done 2>&1 | tee -a z.bench-release-s.log

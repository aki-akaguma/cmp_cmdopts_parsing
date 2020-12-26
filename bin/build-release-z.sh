#!/bin/sh

echo > z.build-release-z.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    env -u CARGO_PROFILE_RELEASE_CODEGEN_UNITS -u CARGO_PROFILE_RELEASE_PANIC -u CARGO_PROFILE_RELEASE_LTO \
      CARGO_PROFILE_RELEASE_OPT_LEVEL="z" \ 
      cargo build --release
  })
  echo
done 2>&1 | tee -a z.build-release-z.log


#!/bin/sh

echo > z.build-release.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    env -u CARGO_PROFILE_RELEASE_OPT_LEVEL -u CARGO_PROFILE_RELEASE_CODEGEN_UNITS -u CARGO_PROFILE_RELEASE_PANIC \
      cargo build --release
      #
      #CARGO_PROFILE_RELEASE_LTO="false" \
  })
  echo
done 2>&1 | tee -a z.build-release.log


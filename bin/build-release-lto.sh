#!/bin/sh

echo > z.build-release-lto.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    env -u CARGO_PROFILE_RELEASE_OPT_LEVEL -u CARGO_PROFILE_RELEASE_CODEGEN_UNITS -u CARGO_PROFILE_RELEASE_PANIC \
      CARGO_PROFILE_RELEASE_LTO="thin" \
      cargo build --release
      #
      #CARGO_PROFILE_RELEASE_LTO="fat" \
      #CARGO_PROFILE_RELEASE_LTO="thin" \
      #CARGO_PROFILE_RELEASE_LTO="false" \
  })
  echo
done 2>&1 | tee -a z.build-release-lto.log


#!/bin/sh

echo > z.build-release.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    env -u CARGO_PROFILE_RELEASE_OPT_LEVEL -u CARGO_PROFILE_RELEASE_CODEGEN_UNITS -u CARGO_PROFILE_RELEASE_PANIC \
      CARGO_BUILD_RUSTFLAGS="-C prefer-dynamic" \
      cargo build --release
  })
  echo
done 2>&1 | tee -a z.build-release.log


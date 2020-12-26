#!/bin/sh

echo > z.build-release-lto-z-musl.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    env -u CARGO_PROFILE_RELEASE_CODEGEN_UNITS -u CARGO_PROFILE_RELEASE_PANIC \
      CARGO_PROFILE_RELEASE_LTO="fat" CARGO_PROFILE_RELEASE_OPT_LEVEL="z" \ 
      cargo build --release --target=x86_64-unknown-linux-musl
  })
  echo
done 2>&1 | tee -a z.build-release-lto-z-musl.log


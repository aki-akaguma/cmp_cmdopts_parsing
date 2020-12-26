#!/bin/sh

BP=$(dirname "$0")

MARK_ARCH="$1"
DYNAMIC="$2"
TARGET_ARCH=$($BP/conv-mark-arch.sh "$MARK_ARCH")

[ -z "$TARGET_ARCH" ] && exit;

echo > z.build-release-target-$MARK_ARCH.log

. ./.names.sh

if [ "$DYNAMIC" = "dynamic" ]; then
for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" &&
    env -u CARGO_PROFILE_RELEASE_CODEGEN_UNITS -u CARGO_PROFILE_RELEASE_PANIC -u CARGO_PROFILE_RELEASE_LTO \
      CARGO_PROFILE_RELEASE_OPT_LEVEL="s" CARGO_BUILD_RUSTFLAGS="-C prefer-dynamic" \
      cargo build --release --target="$TARGET_ARCH" )
  echo
done 2>&1 | tee -a z.build-release-target-$MARK_ARCH.log
else
for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" &&
    env -u CARGO_PROFILE_RELEASE_CODEGEN_UNITS -u CARGO_PROFILE_RELEASE_PANIC -u CARGO_PROFILE_RELEASE_LTO \
      CARGO_PROFILE_RELEASE_OPT_LEVEL="s" \ 
      cargo build --release --target="$TARGET_ARCH" )
  echo
done 2>&1 | tee -a z.build-release-target-$MARK_ARCH.log
fi

#!/bin/sh

BP=$(dirname "$0")

MARK_ARCH="$1"
TARGET_ARCH=$($BP/conv-mark-arch.sh "$MARK_ARCH")

[ -z "$TARGET_ARCH" ] && exit;

. ./.names.sh

a=""
for nm in $NAMES; do
  #echo "$nm :::"
  a="$a \"target/$TARGET_ARCH/release/$nm-one\""
  a="$a \"target/$TARGET_ARCH/release/$nm-curl\""
done
echo "$TARGET_ARCH"
eval rust-viewexec size -fc "$a" | rust-gsub -e "^(.*) [^ ]+/target/$TARGET_ARCH/release/([^ ]+)\$" -f "\$1 \$2 [$MARK_ARCH]"
#eval size "$a" | rust-gsub -e "^(.*)/target/$TARGET_ARCH/release/[^ ]+\$" -f "\$1 [$MARK_ARCH]"

#!/bin/sh

BP=$(dirname "$0")

. ./.names.sh
. ./.targets.sh

for tg in $TARGETS_MUSL; do
  #echo "$tg :::"
  "$BP/vsize-release-target.sh" "$tg"
  echo
done

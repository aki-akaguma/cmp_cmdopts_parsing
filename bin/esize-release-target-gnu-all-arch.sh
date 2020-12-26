#!/bin/sh

BP=$(dirname "$0")

. ./.names.sh
. ./.targets.sh

for tg in $TARGETS_GNU; do
  #echo "$tg :::"
  "$BP/esize-release-target.sh" "$tg"
  echo
done

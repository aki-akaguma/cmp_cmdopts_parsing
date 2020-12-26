#!/bin/sh

TS_SLOTS=6
export TS_SLOTS

BP=$(dirname "$0")

. ./.names.sh
. ./.targets.sh

for tg in $TARGETS_MUSL; do
  #echo "$tg :::"
  tsp "$BP/build-release-target.sh" "$tg" "static"
  #echo
done
echo "finish command queue, do check tsp"

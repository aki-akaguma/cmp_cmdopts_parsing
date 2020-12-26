#!/bin/sh

TS_SLOTS=6
export TS_SLOTS

BP=$(dirname "$0")

. ./.names.sh
. ./.targets.sh

for tg in $TARGETS_GNU; do
  #echo "$tg :::"
  tsp "$BP/build-release-target.sh" "$tg" "dynamic"
  #echo
done
echo "finish command queue, do check tsp"

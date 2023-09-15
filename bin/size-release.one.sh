#!/bin/sh

. ./.names.sh

a=""
for nm in $NAMES; do
  #echo "$A :::"
  a="$a \"target/release/$nm-one\""
done
eval size "$a" | aki-gsub -e "^(.*)target/release/([^ ]+)\$" -f "\$1\$2" \
  > z.size-release.one.log

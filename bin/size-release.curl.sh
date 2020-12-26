#!/bin/sh


. ./.names.sh

a=""
for nm in $NAMES; do
  #echo "$A :::"
  a="$a \"target/release/$nm-curl\""
done
eval size "$a" | rust-gsub -e "^(.*)target/release/([^ ]+)\$" -f "\$1\$2" \
  > z.size-release.curl.log

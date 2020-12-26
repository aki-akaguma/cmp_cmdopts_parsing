#!/bin/sh

. ./.names.sh

a=""
for nm in $NAMES; do
  #echo "$A :::"
  a="$a \"$nm/target/release/$nm-one\""
  a="$a \"$nm/target/release/$nm-curl\""
done
eval rust-viewexec size -fc "$a" | rust-gsub -e "^(.*) [^ ]+/target/release/([^ ]+)\$" -f "\$1 \$2"

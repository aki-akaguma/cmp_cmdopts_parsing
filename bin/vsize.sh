#!/bin/sh

. ./.names.sh

a=""
for nm in $NAMES; do
  #echo "$A :::"
  a="$a \"$nm/target/debug/$nm-one\""
  a="$a \"$nm/target/debug/$nm-curl\""
done
eval rust-viewexec size -fc "$a" | aki-gsub -e "^(.*) [^ ]+/target/debug/([^ ]+)\$" -f "\$1 \$2"

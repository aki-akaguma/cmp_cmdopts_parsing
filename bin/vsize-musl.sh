#!/bin/sh

. ./.names.sh

a=""
for nm in $NAMES; do
  #echo "$A :::"
  a="$a \"$nm/target/x86_64-unknown-linux-musl/release/$nm-one\""
  a="$a \"$nm/target/x86_64-unknown-linux-musl/release/$nm-curl\""
done
eval rust-viewexec size -fc "$a" | rust-gsub -e "^(.*) [^ ]+/target/x86_64-unknown-linux-musl/release/([^ ]+)\$" -f "\$1 \$2"

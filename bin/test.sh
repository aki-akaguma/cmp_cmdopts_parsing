#!/bin/sh

echo > z.test.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    env -u RUSTC_WRAPPER \
      cargo test
    [ "$?" -eq "0" ] || exit "$?"
  })
  [ "$?" -eq "0" ] || exit "$?"
  echo
done 2>&1 | tee -a z.test.log
[ "$?" -eq "0" ] || exit "$?"

cat z.test.log |
rust-grep -v -e "^test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out" |
rust-grep -e "^test result:"

cat z.test.log | rust-grep -e "^error: Could not compile"

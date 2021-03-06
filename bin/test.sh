#!/bin/sh

echo > z.test.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    cargo test
    #env -u RUSTC_WRAPPER
    [ "$?" -eq "0" ] || exit "$?"
  })
  [ "$?" -eq "0" ] || exit "$?"
  echo
done 2>&1 | tee -a z.test.log
[ "$?" -eq "0" ] || exit "$?"

cat z.test.log |
aki-mline -i -e "^test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out" |
aki-mline -e "^test result:"

cat z.test.log | aki-mline -e "^error: Could not compile"

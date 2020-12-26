#!/bin/sh

LIST="
cmp_pure_rust
cmp_pure_rust1
cmp_gumdrop
cmp_optpa_util_4
cmp_optpa_util
cmp_commander
cmp_getopts
cmp_lapp
cmp_args
"

for a in $LIST; do
  rust-viewexec size -f F -D "$a/target/release/$a" | grep -e "::" |\
    rust-grep --color never -e "$a" -e "optcolorwhen" \
      -e "gumdrop" -e "optpa" -e "commander" -e "getopts" -e "lapp" -e "args" |\
    sort -k 4 > "z.$a.txt"
done


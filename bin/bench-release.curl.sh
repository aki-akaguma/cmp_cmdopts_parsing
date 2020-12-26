#!/bin/sh

echo > z.bench-release.curl.log

. ./.names.sh

for nm in $NAMES; do
  echo "$nm :::"
  ( cd "comps/$nm" && {
    taskset --cpu-list 2 \
    env -u RUSTC_WRAPPER -u CARGO_PROFILE_RELEASE_OPT_LEVEL -u CARGO_PROFILE_RELEASE_CODEGEN_UNITS -u CARGO_PROFILE_RELEASE_PANIC \
      cargo bench --bench bench-$nm-curl -- --noplot
      #
      #CARGO_PROFILE_BENCH_LTO="false" \
      #CARGO_PROFILE_BENCH_LTO="fat" \
      #CARGO_PROFILE_BENCH_LTO="thin" \
  })
  echo
done 2>&1 | tee -a z.bench-release.curl.log


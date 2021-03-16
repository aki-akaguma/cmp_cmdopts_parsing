# cmp_cmdopts_parsing

I compared different CLI(Command Line Interface) parsers.
I created programs that imitates `curl` as a subject.
`curl` is a multi-protocol acquisition program with 229 options.
This benchmark program passes 5 flags, 4 options and 1 argument.
The compile option is `--relese` and `LTS=false` is specified.


## Benchmark Results

I benchmarked various command line parsers with commands that emulate `curl` with 229 options.
At a glance

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.907 kc |  334 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    7.273 kc |  385 kib |    5.366 kc |   50 kib |
| cmp_gumdrop        |   14.520 kc |  460 kib |   12.613 kc |  126 kib |
| cmp_pure_rust      |   17.164 kc |  399 kib |   15.257 kc |   64 kib |
| cmp_argh           |   25.888 kc |  419 kib |   23.982 kc |   85 kib |
| cmp_pico_args      |  159.335 kc |  458 kib |  157.428 kc |  124 kib |
| cmp_rustop         |  446.199 kc |  508 kib |  444.292 kc |  174 kib |
| cmp_clap           |  494.022 kc |  931 kib |  492.115 kc |  597 kib |
| cmp_structopt      |  686.683 kc | 1022 kib |  684.776 kc |  687 kib |
| cmp_getopts        |  695.011 kc |  421 kib |  693.104 kc |   86 kib |
| cmp_commander      |  714.452 kc |  434 kib |  712.545 kc |   99 kib |
| cmp_lapp           | 1099.587 kc |  473 kib | 1097.680 kc |  139 kib |
| cmp_args           | 2044.251 kc |  490 kib | 2042.344 kc |  156 kib |
| cmp_app            | 2458.065 kc |  721 kib | 2456.158 kc |  387 kib |

- `kc` is kilo cycles, cycles is cpu clock cycles, lower is better
- `.text` is elf .text section size
- `delta` is the difference from cmp_null_void
- `cmp_null_void` is non parser, support only `--help`, `--version`, and output
- `cmp_pure_rust` is newly written with string match
- compiled by rustc 1.50.0 (cb75ad5db 2021-02-10)
- bench on intel Q6600 @ 2.40GHz

- [clap](https://crates.io/crates/clap) - is the most popular and complete one
- [structopt](https://crates.io/crates/structopt) - a clap parser that uses procedural macros
- [gumdrop](https://crates.io/crates/gumdrop) - a simple parser that uses procedural macros
- [argh](https://crates.io/crates/argh) - minimum code size, procedural macros
- [rustop](https://crates.io/crates/rustop) - traditional macro
- [pico-args](https://crates.io/crates/pico-args) - a simple use
- [getopts](https://crates.io/crates/getopts) - a simple use
- [docopt](https://crates.io/crates/docopt) - a very simple use


### rustc beta Benchmark Results

 i think that beta have a little better benchmark than stable

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.296 kc |  330 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    6.972 kc |  380 kib |    5.676 kc |   50 kib |
| cmp_gumdrop        |   12.096 kc |  447 kib |   10.800 kc |  116 kib |
| cmp_pure_rust      |   14.127 kc |  390 kib |   12.831 kc |   60 kib |
| cmp_argh           |   23.818 kc |  415 kib |   22.522 kc |   85 kib |
| cmp_pico_args      |  148.801 kc |  449 kib |  147.505 kc |  119 kib |
| cmp_rustop         |  442.315 kc |  504 kib |  441.019 kc |  174 kib |
| cmp_clap           |  482.823 kc |  923 kib |  481.527 kc |  593 kib |
| cmp_getopts        |  687.889 kc |  416 kib |  686.592 kc |   86 kib |
| cmp_structopt      |  691.688 kc | 1058 kib |  690.391 kc |  728 kib |
| cmp_commander      |  734.700 kc |  429 kib |  733.404 kc |   99 kib |
| cmp_lapp           | 1106.486 kc |  468 kib | 1105.190 kc |  138 kib |
| cmp_args           | 2055.458 kc |  478 kib | 2054.161 kc |  148 kib |
| cmp_app            | 2416.423 kc |  718 kib | 2415.127 kc |  387 kib |

- compiled by rustc 1.51.0-beta.6 (6a1835ad7 2021-03-12)

## What do you think? :octocat:

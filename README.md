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
| cmp_null_void      |    1.334 kc |  330 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    6.786 kc |  380 kib |    5.452 kc |   50 kib |
| cmp_gumdrop        |   11.413 kc |  447 kib |   10.080 kc |  116 kib |
| cmp_pure_rust      |   13.801 kc |  390 kib |   12.468 kc |   60 kib |
| cmp_argh           |   24.139 kc |  415 kib |   22.805 kc |   85 kib |
| cmp_pico_args      |  152.601 kc |  449 kib |  151.267 kc |  118 kib |
| cmp_rustop         |  444.227 kc |  504 kib |  442.893 kc |  174 kib |
| cmp_clap           |  487.033 kc |  924 kib |  485.699 kc |  594 kib |
| cmp_structopt      |  690.881 kc | 1059 kib |  689.547 kc |  728 kib |
| cmp_getopts        |  694.342 kc |  416 kib |  693.009 kc |   86 kib |
| cmp_commander      |  742.975 kc |  429 kib |  741.642 kc |   99 kib |
| cmp_lapp           | 1136.764 kc |  468 kib | 1135.430 kc |  138 kib |
| cmp_args           | 2059.021 kc |  478 kib | 2057.687 kc |  148 kib |
| cmp_app            | 2429.720 kc |  718 kib | 2428.386 kc |  388 kib |

- `kc` is kilo cycles, cycles is cpu clock cycles, lower is better
- `.text` is elf .text section size
- `Δ` is delta, this is the difference from cmp_null_void
- `cmp_null_void` is non parser, support only `--help`, `--version`, and output
- `cmp_pure_rust` is newly written with string match
- compiled by rustc 1.51.0 (2fd73fabe 2021-03-23)
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
| cmp_null_void      |    1.251 kc |  317 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    5.686 kc |  364 kib |    4.434 kc |   47 kib |
| cmp_gumdrop        |   10.799 kc |  433 kib |    9.547 kc |  116 kib |
| cmp_pure_rust      |   11.928 kc |  378 kib |   10.677 kc |   61 kib |
| cmp_argh           |   24.325 kc |  399 kib |   23.073 kc |   82 kib |
| cmp_pico_args      |  149.897 kc |  433 kib |  148.645 kc |  116 kib |
| cmp_rustop         |  439.588 kc |  486 kib |  438.337 kc |  169 kib |
| cmp_clap           |  496.538 kc |  922 kib |  495.287 kc |  605 kib |
| cmp_getopts        |  685.872 kc |  401 kib |  684.620 kc |   84 kib |
| cmp_structopt      |  701.018 kc | 1066 kib |  699.767 kc |  749 kib |
| cmp_commander      |  738.414 kc |  413 kib |  737.162 kc |   96 kib |
| cmp_lapp           | 1106.056 kc |  454 kib | 1104.805 kc |  137 kib |
| cmp_args           | 2034.455 kc |  459 kib | 2033.203 kc |  142 kib |
| cmp_app            | 2390.462 kc |  691 kib | 2389.210 kc |  374 kib |

- compiled by rustc 1.52.0-beta.2 (4f27db695 2021-03-26)

## What do you think? :octocat:

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
| cmp_null_void      |    1.795 kc |  318 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    7.030 kc |  372 kib |    5.234 kc |   54 kib |
| cmp_gumdrop        |   14.357 kc |  474 kib |   12.562 kc |  156 kib |
| cmp_pure_rust      |   17.833 kc |  385 kib |   16.038 kc |   67 kib |
| cmp_argh           |   26.372 kc |  404 kib |   24.577 kc |   85 kib |
| cmp_pico_args      |  160.434 kc |  413 kib |  158.639 kc |   94 kib |
| cmp_rustop         |  437.199 kc |  495 kib |  435.404 kc |  177 kib |
| cmp_clap           |  589.117 kc |  934 kib |  587.322 kc |  616 kib |
| cmp_getopts        |  683.997 kc |  408 kib |  682.202 kc |   90 kib |
| cmp_structopt      |  723.818 kc | 1015 kib |  722.023 kc |  697 kib |
| cmp_commander      |  752.331 kc |  421 kib |  750.535 kc |  102 kib |
| cmp_lapp           | 1123.426 kc |  461 kib | 1121.631 kc |  142 kib |
| cmp_args           | 2097.303 kc |  459 kib | 2095.508 kc |  140 kib |
| cmp_app            | 2378.484 kc |  707 kib | 2376.689 kc |  388 kib |
| cmp_docopt         | 5849.126 kc | 1690 kib | 5847.331 kc | 1371 kib |

- `kc` is kilo cycles, cycles is cpu clock cycles, lower is better
- `.text` is elf .text section size
- `delta` is the difference from cmp_null_void
- `cmp_null_void` is non parser, support only `--help`, `--version`, and output
- `cmp_pure_rust` is newly written with string match
- compile by rustc 1.48.0 (7eac88abb 2020-11-16)
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
| cmp_null_void      |    1.750 kc |  323 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    8.654 kc |  377 kib |    6.904 kc |   54 kib |
| cmp_gumdrop        |   14.691 kc |  476 kib |   12.941 kc |  153 kib |
| cmp_pure_rust      |   16.508 kc |  389 kib |   14.758 kc |   65 kib |
| cmp_argh           |   26.018 kc |  409 kib |   24.268 kc |   85 kib |
| cmp_pico_args      |  182.805 kc |  421 kib |  181.055 kc |   97 kib |
| cmp_rustop         |  439.241 kc |  498 kib |  437.492 kc |  174 kib |
| cmp_clap           |  555.238 kc |  941 kib |  553.488 kc |  617 kib |
| cmp_structopt      |  698.456 kc | 1022 kib |  696.706 kc |  699 kib |
| cmp_getopts        |  704.658 kc |  412 kib |  702.909 kc |   89 kib |
| cmp_commander      |  766.314 kc |  424 kib |  764.565 kc |  100 kib |
| cmp_lapp           | 1108.724 kc |  464 kib | 1106.974 kc |  140 kib |
| cmp_args           | 2063.641 kc |  463 kib | 2061.892 kc |  140 kib |
| cmp_app            | 2348.568 kc |  713 kib | 2346.818 kc |  389 kib |
| cmp_docopt         | 5625.869 kc | 1693 kib | 5624.119 kc | 1369 kib |

- rustc 1.49.0-beta.5 (b0dc3c6f4 2020-12-22)

## What do you think? /:octocat:

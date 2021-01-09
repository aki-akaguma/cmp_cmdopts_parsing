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
| cmp_null_void      |    1.754 kc |  323 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    6.654 kc |  377 kib |    4.900 kc |   54 kib |
| cmp_gumdrop        |   14.244 kc |  477 kib |   12.490 kc |  153 kib |
| cmp_pure_rust      |   16.439 kc |  389 kib |   14.685 kc |   66 kib |
| cmp_argh           |   26.279 kc |  409 kib |   24.525 kc |   85 kib |
| cmp_pico_args      |  156.589 kc |  421 kib |  154.835 kc |   98 kib |
| cmp_rustop         |  439.899 kc |  498 kib |  438.145 kc |  175 kib |
| cmp_clap           |  562.743 kc |  942 kib |  560.989 kc |  618 kib |
| cmp_structopt      |  676.121 kc | 1023 kib |  674.367 kc |  700 kib |
| cmp_getopts        |  697.585 kc |  412 kib |  695.831 kc |   89 kib |
| cmp_commander      |  762.846 kc |  424 kib |  761.092 kc |  100 kib |
| cmp_lapp           | 1109.102 kc |  464 kib | 1107.348 kc |  140 kib |
| cmp_args           | 2064.578 kc |  464 kib | 2062.824 kc |  140 kib |
| cmp_app            | 2379.029 kc |  714 kib | 2377.275 kc |  390 kib |
| cmp_docopt         | 5714.188 kc | 1694 kib | 5712.434 kc | 1370 kib |

- `kc` is kilo cycles, cycles is cpu clock cycles, lower is better
- `.text` is elf .text section size
- `delta` is the difference from cmp_null_void
- `cmp_null_void` is non parser, support only `--help`, `--version`, and output
- `cmp_pure_rust` is newly written with string match
- compile by rustc 1.49.0 (e1884a8e3 2020-12-29)
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
| cmp_null_void      |    1.918 kc |  335 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    7.221 kc |  382 kib |    5.303 kc |   47 kib |
| cmp_gumdrop        |   14.141 kc |  459 kib |   12.223 kc |  124 kib |
| cmp_pure_rust      |   16.762 kc |  400 kib |   14.844 kc |   64 kib |
| cmp_argh           |   25.602 kc |  419 kib |   23.684 kc |   84 kib |
| cmp_pico_args      |  157.862 kc |  439 kib |  155.944 kc |  104 kib |
| cmp_rustop         |  431.952 kc |  512 kib |  430.034 kc |  177 kib |
| cmp_clap           |  516.139 kc |  931 kib |  514.221 kc |  596 kib |
| cmp_structopt      |  685.031 kc | 1027 kib |  683.113 kc |  692 kib |
| cmp_getopts        |  689.997 kc |  422 kib |  688.078 kc |   87 kib |
| cmp_commander      |  716.954 kc |  434 kib |  715.036 kc |   99 kib |
| cmp_lapp           | 1091.205 kc |  474 kib | 1089.286 kc |  139 kib |
| cmp_args           | 2043.885 kc |  496 kib | 2041.967 kc |  160 kib |
| cmp_app            | 2405.617 kc |  722 kib | 2403.699 kc |  386 kib |
| cmp_docopt         | 5689.437 kc | 1710 kib | 5687.519 kc | 1375 kib |

- compile by rustc 1.50.0-beta.2 (25b3db3aa 2020-12-31)

## What do you think? /:octocat:

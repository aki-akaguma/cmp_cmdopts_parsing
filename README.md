# cmp_cmdopts_parsing

![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]

I compared different CLI(Command Line Interface) parsers.
I created programs that imitates `curl` as a subject.
`curl` is a multi-protocol acquisition program with 229 options.
This benchmark program passes 5 flags, 4 options and 1 argument.
The compile option is `--relese` and `LTS=false` is specified.


## Benchmark Results

I benchmarked various command line parsers with commands that emulate `curl` with 229 options.
At a glance

- compiled by rustc 1.66.0 (69f9c33d7 2022-12-12)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.327 kc |  316 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    6.315 kc |  356 kib |    4.988 kc |   40 kib |
| cmp_pure_rust      |    7.951 kc |  368 kib |    6.624 kc |   52 kib |
| cmp_gumdrop        |   11.346 kc |  432 kib |   10.019 kc |  116 kib |
| cmp_argh           |   20.851 kc |  385 kib |   19.524 kc |   69 kib |
| cmp_pico_args      |   39.187 kc |  393 kib |   37.860 kc |   77 kib |
| cmp_rustop         |  379.726 kc |  465 kib |  378.399 kc |  149 kib |
| cmp_clap           |  415.422 kc |  988 kib |  414.095 kc |  671 kib |
| cmp_clap3          |  495.219 kc |  840 kib |  493.893 kc |  524 kib |
| cmp_structopt      |  553.679 kc |  862 kib |  552.352 kc |  546 kib |
| cmp_getopts        |  637.986 kc |  395 kib |  636.659 kc |   78 kib |
| cmp_commander      |  665.407 kc |  412 kib |  664.080 kc |   95 kib |
| cmp_lapp           | 1115.093 kc |  451 kib | 1113.766 kc |  135 kib |
| cmp_args           | 2101.706 kc |  427 kib | 2100.379 kc |  110 kib |
| cmp_app            | 2192.245 kc |  630 kib | 2190.918 kc |  313 kib |

- compiled by rustc 1.61.0 (fe5b13d68 2022-06-29)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.216 kc |  316 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    5.710 kc |  356 kib |    4.495 kc |   40 kib |
| cmp_pure_rust      |    7.937 kc |  368 kib |    6.721 kc |   52 kib |
| cmp_gumdrop        |    9.957 kc |  432 kib |    8.741 kc |  116 kib |
| cmp_argh           |   23.254 kc |  385 kib |   22.038 kc |   69 kib |
| cmp_pico_args      |   39.693 kc |  393 kib |   38.477 kc |   77 kib |
| cmp_rustop         |  391.021 kc |  465 kib |  389.805 kc |  149 kib |
| cmp_clap           |  426.605 kc |  988 kib |  425.389 kc |  671 kib |
| cmp_clap3          |  516.858 kc |  840 kib |  515.643 kc |  524 kib |
| cmp_structopt      |  595.319 kc |  862 kib |  594.103 kc |  546 kib |
| cmp_getopts        |  646.917 kc |  395 kib |  645.701 kc |   78 kib |
| cmp_commander      |  724.505 kc |  412 kib |  723.289 kc |   95 kib |
| cmp_lapp           | 1091.450 kc |  451 kib | 1090.235 kc |  135 kib |
| cmp_args           | 2015.081 kc |  427 kib | 2013.865 kc |  110 kib |
| cmp_app            | 2175.477 kc |  630 kib | 2174.261 kc |  313 kib |

- compiled by rustc 1.57.0 (f1edd0429 2021-11-29)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.405 kc |  316 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    5.603 kc |  356 kib |    4.197 kc |   40 kib |
| cmp_pure_rust      |    7.845 kc |  368 kib |    6.439 kc |   52 kib |
| cmp_gumdrop        |    8.737 kc |  432 kib |    7.332 kc |  116 kib |
| cmp_argh           |   23.114 kc |  385 kib |   21.708 kc |   69 kib |
| cmp_pico_args      |   41.325 kc |  393 kib |   39.920 kc |   77 kib |
| cmp_rustop         |  394.432 kc |  465 kib |  393.026 kc |  149 kib |
| cmp_clap           |  426.678 kc |  988 kib |  425.273 kc |  671 kib |
| cmp_clap3          |  495.857 kc |  840 kib |  494.452 kc |  524 kib |
| cmp_structopt      |  576.224 kc |  862 kib |  574.818 kc |  546 kib |
| cmp_getopts        |  657.353 kc |  395 kib |  655.948 kc |   78 kib |
| cmp_commander      |  673.761 kc |  412 kib |  672.356 kc |   95 kib |
| cmp_lapp           | 1089.452 kc |  451 kib | 1088.047 kc |  135 kib |
| cmp_args           | 2066.320 kc |  427 kib | 2064.915 kc |  110 kib |
| cmp_app            | 2171.903 kc |  630 kib | 2170.498 kc |  313 kib |

- `kc` is kilo cycles, cycles is cpu clock cycles, lower is better
- `.text` is elf .text section size
- `Δ` is delta, this is the difference from cmp_null_void
- `cmp_null_void` is non parser, support only `--help`, `--version`, and output
- `cmp_pure_rust` is newly written with string match
- bench on intel Q6600 @ 2.40GHz

- [clap](https://crates.io/crates/clap) - is the most popular and complete one
- [structopt](https://crates.io/crates/structopt) - a clap parser that uses procedural macros
- [gumdrop](https://crates.io/crates/gumdrop) - a simple parser that uses procedural macros
- [argh](https://crates.io/crates/argh) - minimum code size, procedural macros
- [rustop](https://crates.io/crates/rustop) - traditional macro
- [pico-args](https://crates.io/crates/pico-args) - a simple use
- [getopts](https://crates.io/crates/getopts) - a simple use
- [docopt](https://crates.io/crates/docopt) - a very simple use


## What do you think? :octocat:

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.57+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg

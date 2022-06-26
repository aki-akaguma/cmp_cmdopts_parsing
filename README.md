# cmp_cmdopts_parsing

I compared different CLI(Command Line Interface) parsers.
I created programs that imitates `curl` as a subject.
`curl` is a multi-protocol acquisition program with 229 options.
This benchmark program passes 5 flags, 4 options and 1 argument.
The compile option is `--relese` and `LTS=false` is specified.


## Benchmark Results

I benchmarked various command line parsers with commands that emulate `curl` with 229 options.
At a glance

- compiled by rustc 1.61.0 (fe5b13d68 2022-05-18)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.200 kc |  350 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    5.760 kc |  391 kib |    4.561 kc |   41 kib |
| cmp_pure_rust      |   10.867 kc |  400 kib |    9.667 kc |   49 kib |
| cmp_gumdrop        |   11.298 kc |  455 kib |   10.099 kc |  105 kib |
| cmp_argh           |   24.086 kc |  438 kib |   22.887 kc |   87 kib |
| cmp_pico_args      |   41.809 kc |  430 kib |   40.609 kc |   79 kib |
| cmp_rustop         |  429.024 kc |  505 kib |  427.825 kc |  155 kib |
| cmp_clap           |  476.143 kc |  922 kib |  474.944 kc |  572 kib |
| cmp_getopts        |  653.458 kc |  438 kib |  652.258 kc |   87 kib |
| cmp_structopt      |  654.494 kc | 1009 kib |  653.295 kc |  658 kib |
| cmp_commander      |  726.200 kc |  450 kib |  725.001 kc |   99 kib |
| cmp_lapp           | 1117.586 kc |  482 kib | 1116.386 kc |  132 kib |
| cmp_args           | 2014.028 kc |  492 kib | 2012.829 kc |  141 kib |
| cmp_app            | 2363.958 kc |  740 kib | 2362.758 kc |  389 kib |

- compiled by rustc 1.56.1 (59eed8a2a 2021-11-01)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.213 kc |  342 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    5.439 kc |  386 kib |    4.226 kc |   43 kib |
| cmp_gumdrop        |   10.686 kc |  456 kib |    9.473 kc |  114 kib |
| cmp_pure_rust      |   11.052 kc |  390 kib |    9.839 kc |   47 kib |
| cmp_argh           |   23.644 kc |  429 kib |   22.431 kc |   86 kib |
| cmp_pico_args      |  152.401 kc |  457 kib |  151.188 kc |  114 kib |
| cmp_rustop         |  429.309 kc |  497 kib |  428.096 kc |  155 kib |
| cmp_clap           |  488.611 kc |  925 kib |  487.399 kc |  583 kib |
| cmp_getopts        |  695.832 kc |  428 kib |  694.620 kc |   85 kib |
| cmp_structopt      |  697.229 kc | 1083 kib |  696.016 kc |  741 kib |
| cmp_commander      |  712.974 kc |  437 kib |  711.762 kc |   94 kib |
| cmp_lapp           | 1113.328 kc |  476 kib | 1112.115 kc |  133 kib |
| cmp_args           | 2017.061 kc |  489 kib | 2015.848 kc |  146 kib |
| cmp_app            | 2348.416 kc |  720 kib | 2347.203 kc |  377 kib |

- compiled by rustc 1.53.0 (53cb7b09b 2021-06-17)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.219 kc |  311 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    5.753 kc |  358 kib |    4.534 kc |   46 kib |
| cmp_gumdrop        |   10.389 kc |  427 kib |    9.170 kc |  116 kib |
| cmp_pure_rust      |   12.047 kc |  372 kib |   10.828 kc |   61 kib |
| cmp_argh           |   24.134 kc |  393 kib |   22.916 kc |   82 kib |
| cmp_pico_args      |  149.519 kc |  428 kib |  148.300 kc |  117 kib |
| cmp_rustop         |  424.860 kc |  479 kib |  423.641 kc |  168 kib |
| cmp_clap           |  495.830 kc |  928 kib |  494.612 kc |  617 kib |
| cmp_getopts        |  681.002 kc |  395 kib |  679.783 kc |   84 kib |
| cmp_structopt      |  687.995 kc | 1056 kib |  686.777 kc |  745 kib |
| cmp_commander      |  737.606 kc |  408 kib |  736.388 kc |   97 kib |
| cmp_lapp           | 1137.391 kc |  450 kib | 1136.172 kc |  139 kib |
| cmp_args           | 2043.305 kc |  455 kib | 2042.086 kc |  144 kib |
| cmp_app            | 2390.787 kc |  690 kib | 2389.569 kc |  379 kib |

- compiled by rustc 1.52.0 (88f19c6da 2021-05-03)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.237 kc |  317 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    5.937 kc |  364 kib |    4.700 kc |   47 kib |
| cmp_gumdrop        |   10.462 kc |  433 kib |    9.225 kc |  116 kib |
| cmp_pure_rust      |   12.555 kc |  378 kib |   11.318 kc |   61 kib |
| cmp_argh           |   23.902 kc |  399 kib |   22.665 kc |   82 kib |
| cmp_pico_args      |  150.100 kc |  434 kib |  148.863 kc |  117 kib |
| cmp_rustop         |  435.712 kc |  487 kib |  434.475 kc |  169 kib |
| cmp_clap           |  499.099 kc |  924 kib |  497.862 kc |  607 kib |
| cmp_getopts        |  681.282 kc |  401 kib |  680.045 kc |   84 kib |
| cmp_structopt      |  726.490 kc | 1069 kib |  725.253 kc |  751 kib |
| cmp_commander      |  727.466 kc |  414 kib |  726.230 kc |   96 kib |
| cmp_lapp           | 1126.964 kc |  454 kib | 1125.727 kc |  137 kib |
| cmp_args           | 2033.918 kc |  460 kib | 2032.681 kc |  142 kib |
| cmp_app            | 2464.854 kc |  692 kib | 2463.617 kc |  375 kib |

- compiled by rustc 1.41.1 (f3e1a954d 2020-02-24)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.193 kc |  248 kib |    0.000 kc |    0 kib |
| cmp_flood_tide     |    7.145 kc |  315 kib |    5.953 kc |   67 kib |
| cmp_gumdrop        |   13.283 kc |  429 kib |   12.090 kc |  180 kib |
| cmp_pure_rust      |   18.806 kc |  340 kib |   17.613 kc |   91 kib |
| cmp_argh           |   25.792 kc |  336 kib |   24.600 kc |   87 kib |
| cmp_pico_args      |  174.907 kc |  375 kib |  173.714 kc |  127 kib |
| cmp_rustop         |  472.151 kc |  442 kib |  470.959 kc |  194 kib |
| cmp_clap           |  585.497 kc |  902 kib |  584.304 kc |  653 kib |
| cmp_getopts        |  675.789 kc |  367 kib |  674.596 kc |  118 kib |
| cmp_structopt      |  741.325 kc |  950 kib |  740.132 kc |  701 kib |
| cmp_commander      | 1038.226 kc |  352 kib | 1037.033 kc |  103 kib |
| cmp_lapp           | 1054.788 kc |  395 kib | 1053.595 kc |  146 kib |
| cmp_args           | 2131.094 kc |  398 kib | 2129.901 kc |  149 kib |
| cmp_app            | 2563.295 kc |  637 kib | 2562.103 kc |  388 kib |

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

# Changelog: cmp_cmdopts_parsing

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
* rename: `config` to `config.toml`
* update crates: regex (1.10), clap, flood-tied, rustop
* rename: `cmp_clap` to `cmp_clap2`

## Fixed
* clippy: clippy::derivable_impls
* clippy: clippy::useless_conversion
* clippy: clippy::unnecessary_get_then_check
* clippy: clippy::map_clone
* clippy: clippy::assigning_clones
* clippy: clippy::needless_borrow
* clippy: clippy::question_mark


## [0.1.18] (2023-09-16)
### Added
* badges into `README.md`
* added `cmp_clap4` into `comp`

### Changed
* reformat `CHANGELOG.md`
* update benchmark results
* update crates: criterion(0.5), clap(4.4.3), ...

### Removed
* remove short option: `-h` of curl command

## [0.1.17] (2023-01-02)
### Changed
* update benchmark results
* update crates: criterion(0.4)

## [0.1.16] (2022-06-29)
### Added
* cmp_clap3

## [0.1.15] (2022-06-26)
### Changed
* update depends: argh(0.1.7), clap(2.34.0), commander(0.1.5)
* update depends: flood-tide(0.2.5), gumdrop(0.8.1), pico_args(0.5.0)
* update depends: struct_opt(0.3.26)
* changes to edition 2021

## [0.1.14] (2021-06-29)
### Changed
* update depends: pico-args(0.4.2), rustc_version(0.4)

## [0.1.13] (2021-05-09)
### Changed
* update depends: flood-tide(0.2.2)
* update depends: flood-tide-gen(0.1.13), regex(1.5.4)

## [0.1.12] (2021-04-25)
### Changed
* update depends: flood-tide-gen(0.1.12), regex(1.4.6)

## [0.1.11] (2021-04-19)
### Changed
* update depends: flood-tide-gen(0.1.10)

## [0.1.10] (2021-04-04)
### Changed
* update depends: anyhow(1.0.40): Reduce memory footprint of errors on Rust versions 1.51+
* update depends: js-sys, libc, memoffset, serde ...
* update depends: flood-tide(0.2.0)
* update README.md: the results of compiled by rustc 1.51.0 (2fd73fabe 2021-03-23)

## [0.1.9] (2021-03-16)
### Changed
* update depends
* update crates: docopt 1.1.0 -> 1.1.1
* update crates: flood-tide 0.1.19 -> 0.1.21
* update crates: pico-args 0.3.4 -> 0.4.0

### Removed
* remove docopt: because of unmaintained

## [0.1.8] (2021-01-03)
### Changed
* update rustc 1.49.0 (e1884a8e3 2020-12-29)
* update crates

## [0.1.7] (2020-12-26)
### Changed
* refine for github

## 0.1.6 (2020-12-25)
### Added
* cmp_flood_tide

## 0.1.5 (2020-12-21)
### Changed
* update optpa-util-5

## 0.1.4 (2020-12-18)
### Added
* text-curl.rs

### Changed
* copy some crates on local repogitory to comps/common
* refactoring source code
* update crates and benchmark result

## 0.1.3 (2020-12-04)
### Added
* xtask shape_benchmark_results
* cmp_null_void

## 0.1.2 (2020-12-04)
### Changed
* move cmp_* to comps

## 0.1.1 (2020-12-04)
### Added
* top level workspace

## 0.1.1 (2020-12-04)
* first commit

[Unreleased]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.18..HEAD
[0.1.18]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/cmp_cmdopts_parsing/releases/tag/v0.1.7

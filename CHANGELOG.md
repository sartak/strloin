# Changelog

## next

- _breaking_: upgrade to 2024 edition, which bumps MSRV to 1.85

## [0.2.0] - 2024-07-23

- _breaking_: remove optional `beef` feature since it's not semver compatible
- _breaking_: remove `Ranges::from_range(0..9)` in favor of `Ranges::from(0..9)`

## [0.1.4] - 2023-09-09

- add a `push_unchecked` method to `Ranges`

## [0.1.3] - 2023-09-09

- split into modules
- add a `Ranges` object and add to example benchmark
- implement more common traits

## [0.1.2] - 2023-09-08

- add benchmark result
- re-export Cow, Owned, and Borrowed for different COW implementations
- add optional `beef` feature for [beef](https://crates.io/crates/beef) crate

## [0.1.1] - 2023-09-06

- fix case where invalid ranges could return bogus results

## [0.1.0] - 2023-09-06

- initial release

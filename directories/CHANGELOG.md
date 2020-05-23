# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
# Guiding Principles

* Changelogs are for _humans_, not machines.
* There should be an entry for every single version.
* The same types of changes should be grouped.
* Versions and sections should be linkable.
* The latest version comes first.
* The release date of each version is displayed.
* Mention whether you follow Semantic Versioning.

# Types of changes

* `Added` for new features.
* `Changed` for changes in existing functionality.
* `Deprecated` for soon-to-be removed features.
* `Removed` for now removed features.
* `Fixed` for any bug fixes.
* `Security` in case of vulnerabilities.
 -->

## [1.0.1]
### Fixed
* Relax pinning `cfg-if` at `0.1.9`. Previously we inherited that from upstream repository when forking.
  The original author wanted to keep minimum Rust version at 1.13 which we don't anymore.

## [1.0.0] - 2020-05-17

This is the re-publish of `directories` crate as is.

[1.0.1]: https://github.com/xdg-rs/dirs/compare/directories-v1.0.0...directories-v1.0.1
[1.0.0]: https://github.com/xdg-rs/dirs/releases/tag/directories-v1.0.0

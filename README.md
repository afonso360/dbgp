# dbgp [![travis-badge][]][travis] [![appveyor-badge][]][appveyor] [![coveralls-badge][]][coveralls] [![dependencyci-badge][]][dependencyci] [![license-badge][]][license] [![docs-badge][]][docs]
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fafonso360%2Fdbgp.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fafonso360%2Fdbgp?ref=badge_shield)


Implementation of the dbgp protocol

This intends to comply with the protocol available on date: 2017-04-04 on [this][dbgp] website

The protocol is also available in this [git repository][dbgp-git]

## Fuzzing

To run fuzzing on this crate install `cargo-fuzz` and run

`cargo fuzz run -j4 fuzz_packet_parse -- -max_len=256`

## TODO
 - Quickcheck roundtrip tests for packet (de)serialization
 - We should consider changing to `Cow` instead of `String`


## License

dbgp is licensed under GPL General Public License 2.0 only with a Linking exception

This means that you can link it with your program even if its license is not GPL

Read [LICENSE][license] for more information.

[travis-badge]: https://img.shields.io/travis/afonso360/dbgp/master.svg?style=flat-square
[appveyor-badge]: https://img.shields.io/appveyor/ci/afonso360/dbgp/master.svg?style=flat-square
[coveralls-badge]: https://img.shields.io/coveralls/afonso360/dbgp/master.svg?style=flat-square
[dependencyci-badge]: https://dependencyci.com/github/afonso360/dbgp/badge?style=flat-square
[license-badge]: https://img.shields.io/badge/license-GPLv2%20With%20Linking%20exception-blue.svg?style=flat-square
[docs-badge]: https://img.shields.io/badge/docs-0.0.1-blue.svg?style=flat-square
[travis]: https://travis-ci.org/afonso360/dbgp
[appveyor]: https://ci.appveyor.com/project/afonso360/dbgp
[coveralls]: https://coveralls.io/github/afonso360/dbgp
[docs]: https://docs.rs/dbgp/0.0.1/dbgp/
[license]: LICENSE
[dbgp]: https://xdebug.org/docs-dbgp.php#id21
[dbgp-git]: https://github.com/derickr/dbgp
[dependencyci]: https://dependencyci.com/github/afonso360/dbgp


[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fafonso360%2Fdbgp.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fafonso360%2Fdbgp?ref=badge_large)
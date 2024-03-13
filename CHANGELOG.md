# Changelog

## [0.4.0](https://github.com/izyuumi/html2md-rs/compare/v0.3.0...v0.4.0) (2024-03-13)


### Features

* add br tag support [#11](https://github.com/izyuumi/html2md-rs/issues/11) ([cf23033](https://github.com/izyuumi/html2md-rs/commit/cf230339caaa7cc30513bd73337ccf38d2744cc4))
* extend parser and structs, update tests ([2bdb52a](https://github.com/izyuumi/html2md-rs/commit/2bdb52aaeb8da9b47f86317f7be3c4769aebacf5))
* **to_md:** add safe_from_html_to_md ([2813fef](https://github.com/izyuumi/html2md-rs/commit/2813fef7254f1d7d83a8da58249c4016de6539cb))


### Bug Fixes

* adjust newline handling and code block formatting ([e6ae0bb](https://github.com/izyuumi/html2md-rs/commit/e6ae0bb365facec0501edf53105ce1ffaf2ad5d3))

## [0.3.0](https://github.com/izyuumi/html2md-rs/compare/v0.2.0...v0.3.0) (2024-03-08)


### Features

* **ci:** add crates release to release workflow ([a77f946](https://github.com/izyuumi/html2md-rs/commit/a77f9464e0050e0ffe0e8e1f2704f94e3067f02a))
* **parser:** handle parsing errors ([fcb54e4](https://github.com/izyuumi/html2md-rs/commit/fcb54e4281b1a24592838f0b54717cb88bb4ba4b))

## [0.2.0](https://github.com/izyuumi/html2md-rs/compare/v0.1.8...v0.2.0) (2024-03-08)


### Features

* add support for hr tag ([af68cd2](https://github.com/izyuumi/html2md-rs/commit/af68cd2f68eef50bfa6279eede72292dc3e3e696))
* **ci:** add release-please gh action ([aa1a000](https://github.com/izyuumi/html2md-rs/commit/aa1a00013f317c9de18122623934ec8980f87b1f))
* **parser:** add support for self closing tags ([8516f7e](https://github.com/izyuumi/html2md-rs/commit/8516f7ec20af69ac3cfde52c21d4485dee4b714a))


### Bug Fixes

* **parser:** fix an issue where closing tags are not parsed properly ([5d85848](https://github.com/izyuumi/html2md-rs/commit/5d8584884a69c366ed246cdb8667a8846fd77b84))

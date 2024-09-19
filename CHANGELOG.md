# Changelog

## [0.9.2](https://github.com/izyuumi/html2md-rs/compare/v0.9.1...v0.9.2) (2024-09-19)


### Bug Fixes

* safely parse angle brackets in attribute values ([4533121](https://github.com/izyuumi/html2md-rs/commit/4533121bb30752f17284d0a5975dd1fddf82a801)), closes [#31](https://github.com/izyuumi/html2md-rs/issues/31)

## [0.9.1](https://github.com/izyuumi/html2md-rs/compare/v0.9.0...v0.9.1) (2024-06-21)


### Performance Improvements

* **to_md:** use pattern binding for rendering headers ([ded0f3b](https://github.com/izyuumi/html2md-rs/commit/ded0f3bc5eb89b06e4cdc4304d51b21a56cae38d))

## [0.9.0](https://github.com/izyuumi/html2md-rs/compare/v0.8.2...v0.9.0) (2024-06-21)


### Features

* custom configuration for how to render md ([2fd820a](https://github.com/izyuumi/html2md-rs/commit/2fd820af919e466f7ae9d2c16edfb0a4d67bb7d7)), closes [#28](https://github.com/izyuumi/html2md-rs/issues/28)


### Bug Fixes

* fix cargo clippy issues ([32b5a9c](https://github.com/izyuumi/html2md-rs/commit/32b5a9c83e21a578ad5024531c08be6bc3f1258d))

## [0.8.2](https://github.com/izyuumi/html2md-rs/compare/v0.8.1...v0.8.2) (2024-04-26)


### ⚠ BREAKING CHANGES

* rename NodeType::from_str() with NodeType::from_tag_str

### Bug Fixes

* rename NodeType::from_str() with NodeType::from_tag_str ([04a058d](https://github.com/izyuumi/html2md-rs/commit/04a058dd2590b79630ff97e9d345746d255cb482))


### Miscellaneous Chores

* release 0.8.2 ([8e7a8cf](https://github.com/izyuumi/html2md-rs/commit/8e7a8cf1c5c57ca3649ff8a5df89fc1a61ed2d45))

## [0.8.1](https://github.com/izyuumi/html2md-rs/compare/v0.8.0...v0.8.1) (2024-04-07)


### Bug Fixes

* allow spaces between equal sign and value ([f604f81](https://github.com/izyuumi/html2md-rs/commit/f604f8108a19ba177535565025d69437c7f60bfa)), closes [#25](https://github.com/izyuumi/html2md-rs/issues/25)

## [0.8.0](https://github.com/izyuumi/html2md-rs/compare/v0.7.2...v0.8.0) (2024-04-03)


### ⚠ BREAKING CHANGES

* enhance HTML attribute parsing and representation

### Features

* enhance HTML attribute parsing and representation ([ec66ebc](https://github.com/izyuumi/html2md-rs/commit/ec66ebccec892ed93a90f91f512aaf306afef859))
* support for boolean attributes ([8ad63bb](https://github.com/izyuumi/html2md-rs/commit/8ad63bbe68fe20a592cdaacb717f0dc9120428e8))


### Bug Fixes

* add deprecated flag to `from_html_to_md` ([eb56449](https://github.com/izyuumi/html2md-rs/commit/eb5644908496e93e3ffb7f2e27e626c7e4c9ff99))
* support for non-quoted attribute values ([956af83](https://github.com/izyuumi/html2md-rs/commit/956af83e2e455557734a27100156d3963d191293)), closes [#23](https://github.com/izyuumi/html2md-rs/issues/23)

## [0.7.2](https://github.com/izyuumi/html2md-rs/compare/v0.7.1...v0.7.2) (2024-04-01)


### Bug Fixes

* set `expect_quotation_mark` flag to false ([d313e94](https://github.com/izyuumi/html2md-rs/commit/d313e94a2c7d9b4a2ca55ecaa1f68e01dff8807f)), closes [#21](https://github.com/izyuumi/html2md-rs/issues/21)

## [0.7.1](https://github.com/izyuumi/html2md-rs/compare/v0.6.0...v0.7.1) (2024-04-01)


### Features

* add support for comments ([cdeedee](https://github.com/izyuumi/html2md-rs/commit/cdeedee239e1a992e40d459a848b8c68ca54d2b4))
* add support for metadata tags ([33a27ba](https://github.com/izyuumi/html2md-rs/commit/33a27ba99434df9340a9823f7cfee1a023824c22)), closes [#16](https://github.com/izyuumi/html2md-rs/issues/16)


### Bug Fixes

* add deprecated flag to `parse_html` as it panics ([af0dfe9](https://github.com/izyuumi/html2md-rs/commit/af0dfe943b60af08e36440bb5f14ef10074954de))
* handle unclosed HTML tags and ignore title tag content ([be05a43](https://github.com/izyuumi/html2md-rs/commit/be05a437e484dbc0a927534d0975655dd8d1284d))
* parse `=` in attribute value properly ([5060304](https://github.com/izyuumi/html2md-rs/commit/5060304a65aceddd5099b4febfa8c6067088e9a4)), closes [#19](https://github.com/izyuumi/html2md-rs/issues/19)
* remove deprecated `parse_html` from tests ([36ec69b](https://github.com/izyuumi/html2md-rs/commit/36ec69bc5791df2e962a76b65a7b5038934f79a0))


### Miscellaneous Chores

* release 0.7.1 ([c59984d](https://github.com/izyuumi/html2md-rs/commit/c59984dd532eb1ec523a3ac891ef483f7f73d279))

## [0.6.0](https://github.com/izyuumi/html2md-rs/compare/v0.5.1...v0.6.0) (2024-03-23)


### Features

* enhance Node struct and improve HTML to markdown conversion ([d11981b](https://github.com/izyuumi/html2md-rs/commit/d11981bfdb38d5bea1e2422dd50923233cc27038))
* support OL start attribute ([bc9b8da](https://github.com/izyuumi/html2md-rs/commit/bc9b8da869bcb28a81bded829b869eeef34a33e7))

## [0.5.1](https://github.com/izyuumi/html2md-rs/compare/v0.5.0...v0.5.1) (2024-03-13)


### Bug Fixes

* remove println calls from parser ([dd6fdb0](https://github.com/izyuumi/html2md-rs/commit/dd6fdb02ea58ffff3f19ca2121eb1f294aa04248))

## [0.5.0](https://github.com/izyuumi/html2md-rs/compare/v0.4.0...v0.5.0) (2024-03-13)


### Features

* add br tag support [#11](https://github.com/izyuumi/html2md-rs/issues/11) ([cf23033](https://github.com/izyuumi/html2md-rs/commit/cf230339caaa7cc30513bd73337ccf38d2744cc4))
* add from_html_to_md() to parse from html to md in one function ([38bfa83](https://github.com/izyuumi/html2md-rs/commit/38bfa83da986e021516be33d787a28c7ff0456c1))
* add support for A tag ([205fc7c](https://github.com/izyuumi/html2md-rs/commit/205fc7cacdd72f10c4e33c06a796d76d7a56c795))
* add support for code blocks ([ea913cd](https://github.com/izyuumi/html2md-rs/commit/ea913cdc892db52d524e0edc5b022189dee2172d))
* add support for hr tag ([af68cd2](https://github.com/izyuumi/html2md-rs/commit/af68cd2f68eef50bfa6279eede72292dc3e3e696))
* add support for ol, ul, and li ([123bd03](https://github.com/izyuumi/html2md-rs/commit/123bd034947dd1080f4aa05db34073baf576999f))
* add to_md feature ([35ccd20](https://github.com/izyuumi/html2md-rs/commit/35ccd20560da24ebaa87dea8594e003f93486491))
* **ci:** add crates release to release workflow ([a77f946](https://github.com/izyuumi/html2md-rs/commit/a77f9464e0050e0ffe0e8e1f2704f94e3067f02a))
* **ci:** add release-please gh action ([aa1a000](https://github.com/izyuumi/html2md-rs/commit/aa1a00013f317c9de18122623934ec8980f87b1f))
* create basic parser ([c74b6d8](https://github.com/izyuumi/html2md-rs/commit/c74b6d8e1beb12dce48f40f3e8cf73a3a81c2acd))
* extend parser and structs, update tests ([2bdb52a](https://github.com/izyuumi/html2md-rs/commit/2bdb52aaeb8da9b47f86317f7be3c4769aebacf5))
* **parser:** add support for self closing tags ([8516f7e](https://github.com/izyuumi/html2md-rs/commit/8516f7ec20af69ac3cfde52c21d4485dee4b714a))
* **parser:** handle parsing errors ([fcb54e4](https://github.com/izyuumi/html2md-rs/commit/fcb54e4281b1a24592838f0b54717cb88bb4ba4b))
* **tests:** add tests for parser and to_md ([4341024](https://github.com/izyuumi/html2md-rs/commit/4341024c0e24eab68a7cddbc52f52ca2d983e8fe))
* **to_md:** add safe_from_html_to_md ([2813fef](https://github.com/izyuumi/html2md-rs/commit/2813fef7254f1d7d83a8da58249c4016de6539cb))


### Bug Fixes

* add new line after paragraphs ([c629214](https://github.com/izyuumi/html2md-rs/commit/c629214d6e3fc122d6852538ffb0b7e21215f51d))
* adjust newline handling and code block formatting ([e6ae0bb](https://github.com/izyuumi/html2md-rs/commit/e6ae0bb365facec0501edf53105ce1ffaf2ad5d3))
* empty paragraphs returns nothing ([359ff4e](https://github.com/izyuumi/html2md-rs/commit/359ff4ecc30822a1f698ab2ffdf0369c41516fcf))
* empty paragraphs should not return anything ([1f55dce](https://github.com/izyuumi/html2md-rs/commit/1f55dceb56b4e6ca5647f489317f5d686f27e1d8))
* paragraphs is followed by one new line ([c69b43b](https://github.com/izyuumi/html2md-rs/commit/c69b43b8c722addd556a36ab7ac785789410b645))
* paragraphs should close with \n\n ([5a247dd](https://github.com/izyuumi/html2md-rs/commit/5a247dd1283ce6f1111252443f65ad80879f0132))
* **parser:** fix an issue where closing tags are not parsed properly ([5d85848](https://github.com/izyuumi/html2md-rs/commit/5d8584884a69c366ed246cdb8667a8846fd77b84))

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

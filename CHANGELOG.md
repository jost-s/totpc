# Changelog

## [2.0.0](https://github.com/jost-s/totpc/compare/v1.1.0...v2.0.0) (2026-10-23)


### ⚠ BREAKING CHANGES

* suppress gpg output ([#4](https://github.com/jost-s/totpc/issues/4))
* **crate:** rename to totpc

### Features

* add bash & zsh completion ([e21442c](https://github.com/jost-s/totpc/commit/e21442c6e2e557a765d6c5c4683ede9ac067d062))
* **command:** add delete command ([2528c78](https://github.com/jost-s/totpc/commit/2528c78217f8d150c280990ac638b397680b44c4))
* **command:** add list ([38d3123](https://github.com/jost-s/totpc/commit/38d3123349a18316004dd9793a7fe25b7bd4a899))
* **command:** add shortcuts ([4772203](https://github.com/jost-s/totpc/commit/4772203aabe44f432b8453041d99a9e1589e6bc2))
* **command:** add shortcuts ([7a3916b](https://github.com/jost-s/totpc/commit/7a3916b86675c497785617b5dda1f16a5ec2d4e0))
* **command:** add update command ([5175a8b](https://github.com/jost-s/totpc/commit/5175a8bc6b0ed44e95a98f3d494464cac02b1539))
* **command:** implement delete ([66b7834](https://github.com/jost-s/totpc/commit/66b7834aff33010cebc03837e5baeb443b7d9374))
* **init:** implement save & compute ([8bbde9d](https://github.com/jost-s/totpc/commit/8bbde9d299b51fdc71d3c1f0933ab9fbf22900c9))
* **keys:** encrypt all keys with gpg ([e2f49ff](https://github.com/jost-s/totpc/commit/e2f49ff5e67e6aa2980edc3132a2b0f4b0364dcd))
* **keys:** encrypt all keys with gpg ([b381bd2](https://github.com/jost-s/totpc/commit/b381bd26fbbb6c6b71d29acad631525e62fcd8b4))
* **save:** test key for valid base32 encoding ([5e2d940](https://github.com/jost-s/totpc/commit/5e2d9403918f0175e7a06270ce4b0098dabb823a))
* suppress gpg output ([#4](https://github.com/jost-s/totpc/issues/4)) ([008534a](https://github.com/jost-s/totpc/commit/008534a2e223f936e7dee23fee924d8172d545a0))
* **totp:** add read command ([05a3a9f](https://github.com/jost-s/totpc/commit/05a3a9f60df94a63023d84e131305ed426c5ac9b))


### Bug Fixes

* **delete:** append newline at end of file ([f2c7688](https://github.com/jost-s/totpc/commit/f2c7688839c7e7686d5f211b83b27fd1eac06267))
* **file:** remove additional newline after entry ([73e6bcf](https://github.com/jost-s/totpc/commit/73e6bcfdb484571c7a0ca9d9d1d3e40a5ddf1dfc))


### Refactoring

* **crate:** rename to totpc ([8d754e8](https://github.com/jost-s/totpc/commit/8d754e86f07c38b65561bcf3136e121a7a691b33))
* **delete:** improve algorithm ([b03ecc1](https://github.com/jost-s/totpc/commit/b03ecc156ff7d9915630c5c7a14f42ccb8547ee7))
* **entry:** introduce entry struct ([610f2cf](https://github.com/jost-s/totpc/commit/610f2cf3838623273753846f472f35be8007cd3d))
* **file:** extract shared fn to find entry ([7966db3](https://github.com/jost-s/totpc/commit/7966db36ecb3be3408975c17a30be335e5870f20))
* **format:** inline all string literals ([81104f1](https://github.com/jost-s/totpc/commit/81104f1d49093b888b9c94de9da4e531e07c704d))
* **package:** rename to totp_store ([0687fbf](https://github.com/jost-s/totpc/commit/0687fbfa68bd3cfb1736cb9b3eafe9cfc354bc25))
* **totp:** default to list command ([12bd711](https://github.com/jost-s/totpc/commit/12bd7114f22303053776f42b93e45611e5396a07))


### Build System

* **nix:** add nix dev shell ([8e3d959](https://github.com/jost-s/totpc/commit/8e3d9591ef1d7c2d5af9df3bd12cfc5a0b6e3bb8))
* set release please version to 1.1.0 ([19e3dd9](https://github.com/jost-s/totpc/commit/19e3dd9486615fb7e41eab3104a2757a306e1589))


### Continuous Integration

* add badge ([#2](https://github.com/jost-s/totpc/issues/2)) ([9f1e217](https://github.com/jost-s/totpc/commit/9f1e217cb38672c5038f5f54143d39c262e95a4b))
* add release-please config ([7ec2c05](https://github.com/jost-s/totpc/commit/7ec2c0545fee39b0c414a6b3bed4fa98a83bfca7))
* add release-please workflow ([6d20bea](https://github.com/jost-s/totpc/commit/6d20bea506c1692b19eae489904c7412e8679a84))
* add release-please-manifest ([8182896](https://github.com/jost-s/totpc/commit/8182896cf6097bbe1bcfdf6c224cd184b0eba317))
* enable test and builds on ubuntu ([8fd6483](https://github.com/jost-s/totpc/commit/8fd648330f31c3f2a0d4690ae1f7d4371644095b))
* **test:** add test pipeline ([#1](https://github.com/jost-s/totpc/issues/1)) ([7ac5e9a](https://github.com/jost-s/totpc/commit/7ac5e9a7fa681c1c23b6a359f8a1d73a02b1bd43))
* **test:** update checkout action to v7 ([75cdfdf](https://github.com/jost-s/totpc/commit/75cdfdfce407d14fb4be8f6ab92feae77e5ce15b))


### Tests

* **integration:** add delete test ([4a2f893](https://github.com/jost-s/totpc/commit/4a2f893076dd4acd1793f93e9e3841b2e6f1cea7))
* **integration:** move all to same module ([6ca50f6](https://github.com/jost-s/totpc/commit/6ca50f62bfdc9a724c7ea4cb4135fa84ba8a3b16))
* **integration:** remove tests using actual file ([1b21298](https://github.com/jost-s/totpc/commit/1b212989cb1d4b4d0b0f2dcda3815fba2010c45e))


### Documentation

* add docs comments ([#3](https://github.com/jost-s/totpc/issues/3)) ([fa9ca75](https://github.com/jost-s/totpc/commit/fa9ca7587d4433a3e1478ea079e6458f364ddae7))
* add example to run ([83f61e9](https://github.com/jost-s/totpc/commit/83f61e9f0b656284d5cd5bb4bee4de34a6d680e2))
* **cargo:** fix keyword ([754767b](https://github.com/jost-s/totpc/commit/754767b66e44b7045ed44486cf9faa57215f5790))
* **readme:** add installation and usage instructions ([5c84848](https://github.com/jost-s/totpc/commit/5c84848da17a102b49f200d5a8a25627fded1d15))
* update cargo metadata ([7f4f3c2](https://github.com/jost-s/totpc/commit/7f4f3c2753dd3ccdfe965dfc6e9ff8dc923755cd))


### Chores

* delete commented out code ([90d369d](https://github.com/jost-s/totpc/commit/90d369dded2774c280f0f14943c555a457d7c65c))
* **gitignore:** ignore txt and nix files ([1ec8a6c](https://github.com/jost-s/totpc/commit/1ec8a6c518253fcf179867e9af9b48b6f4c85ac3))
* **release:** update deps & release v0.2.0 ([96a93bc](https://github.com/jost-s/totpc/commit/96a93bc0437e0ab084c6dca64997b2f9ab1da0a2))

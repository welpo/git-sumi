# Changelog

Welcome to the changelog for git-sumi. This document aims to provide a comprehensive list of all notable changes made to the project, organised chronologically by release version.

We use Semantic Versioning (SemVer) for our version numbers, formatted as MAJOR.MINOR.PATCH. Major version changes involve significant (breaking) changes, minor versions introduce features and improvements in a backward compatible manner, and patch versions are for bug fixes and minor tweaks.

## [0.2.0](https://github.com/welpo/git-sumi/compare/v0.1.0..v0.2.0) - 2025-08-30

### ✨ Features

- Add option to remove header pattern before validation ([#305](ettps://github.com/welpo/git-sumi/issues/305)) by [@welpo](https://github.com/welpo)

### 🔧 Miscellaneous tasks

- Fix test typos ([#280](https://github.com/welpo/git-sumi/issues/280)) by [@kianmeng](https://github.com/kianmeng)

### 👥 New contributors

🫶 [@kianmeng](https://github.com/kianmeng) made their first contribution in [#280](https://github.com/welpo/git-sumi/pull/280)

## [0.1.0](https://github.com/welpo/git-sumi/compare/v0.0.9..v0.1.0) - 2025-07-05

### ✨ Features

- Detect non-imperatives w/ prefixes ([#269](https://github.com/welpo/git-sumi/issues/269)) by [@welpo](https://github.com/welpo)
- Update gitmoji to v3.15 ([3e43dc5](https://github.com/welpo/git-sumi/commit/3e43dc5047374e8cee0562d3be03b546919610d1)) by [@welpo](https://github.com/welpo)
- Add pre-commit hook config ([#199](https://github.com/welpo/git-sumi/issues/199)) by [@TribuneX](https://github.com/TribuneX) and [@welpo](https://github.com/welpo)
- Support reading commit message from a file ([#198](https://github.com/welpo/git-sumi/issues/198)) by [@welpo](https://github.com/welpo)
- Expand non-imperative verbs ([08a0429](https://github.com/welpo/git-sumi/commit/08a04298b7a0576d68537e19b97961416943ae11)) by [@welpo](https://github.com/welpo)

### 🐛 Bug fixes

- Remove `styling` from non-imperative verbs list ([d9b42d8](https://github.com/welpo/git-sumi/commit/d9b42d8b1bd65cb114e67dcc7635d49b6399c3bc)) by [@welpo](https://github.com/welpo)

### 📝 Documentation

- *(README)* Add quick start guide ([3eb3d8c](https://github.com/welpo/git-sumi/commit/3eb3d8ccdafc3a145d2622d299469983b724f52c)) by [@welpo](https://github.com/welpo)
- *(website)* Make commit-msg handle first commit ([e982449](https://github.com/welpo/git-sumi/commit/e982449c3d58673df417164a0c16e0a13627959c)) by [@welpo](https://github.com/welpo)
- Add uv to installation instructions ([cd3c460](https://github.com/welpo/git-sumi/commit/cd3c4604c7ce765169a61ac34692e0923aad5471)) by [@welpo](https://github.com/welpo)
- Add Chocolatey as install option ([20bd2db](https://github.com/welpo/git-sumi/commit/20bd2db1fb683445fb452ff3a233e555556e8531)) by [@welpo](https://github.com/welpo)

### ♻️ Refactor

- Replace lazy_static with LazyLock ([1b164a7](https://github.com/welpo/git-sumi/commit/1b164a71e63e4959fbb0faa64ad84b8203a066f2)) by [@welpo](https://github.com/welpo)

### 🔧 Miscellaneous tasks

- *(CI)* Add concurrency control to GH action ([779172c](https://github.com/welpo/git-sumi/commit/779172c34ff7a380a068eedc3ffc7b10073ac51a)) by [@welpo](https://github.com/welpo)
- *(CI)* Allow longer PR titles for dep updates ([3aafa53](https://github.com/welpo/git-sumi/commit/3aafa533107c424b35c9175437998952eefb15c5)) by [@welpo](https://github.com/welpo)
- *(CI)* Allow longer PR titles for dep updates ([4c32b32](https://github.com/welpo/git-sumi/commit/4c32b32cee31ae0675bd1d630b30d3bd269b4b92)) by [@welpo](https://github.com/welpo)
- *(CONTRIBUTING)* Add local web dev instructions ([d7d2c14](https://github.com/welpo/git-sumi/commit/d7d2c1414460fc933ef69a474bda7f8380916a66)) by [@welpo](https://github.com/welpo)
- *(README)* Close <a> tag for GH release badge ([6a0c9da](https://github.com/welpo/git-sumi/commit/6a0c9da33120cddaa968f2c14744e8cdf1919954)) by [@welpo](https://github.com/welpo)
- *(README)* Fix badges layout ([3d7a013](https://github.com/welpo/git-sumi/commit/3d7a01364f8ae0ee9f94ab2040ecbefb0a8f0d94)) by [@welpo](https://github.com/welpo)
- *(README)* Add crates size badge ([9cb8181](https://github.com/welpo/git-sumi/commit/9cb8181fca1a0b2ade11e984aa6f416dafc92715)) by [@welpo](https://github.com/welpo)
- *(README)* Add PyPI version badge ([3507852](https://github.com/welpo/git-sumi/commit/35078520cfa65a7237d2eb7f6f672c0ead02d076)) by [@welpo](https://github.com/welpo)
- *(README)* Fix mismatched h3 tag ([ad7c8cb](https://github.com/welpo/git-sumi/commit/ad7c8cb01d20421dcef0005665e1c6f3556dc784)) by [@welpo](https://github.com/welpo)
- *(deps)* Remove local release script ([cae59a1](https://github.com/welpo/git-sumi/commit/cae59a1d893197f6dc965e6ce624f95d8487b469)) by [@welpo](https://github.com/welpo)
- *(deps)* Replace local release script w/ git submodule ([212321b](https://github.com/welpo/git-sumi/commit/212321b6bc045bc218678fcd0abc1c9006c6db1f)) by [@welpo](https://github.com/welpo)
- *(release)* Auto-update Cargo.toml version ([648f2fd](https://github.com/welpo/git-sumi/commit/648f2fdfdc687eb8fc83fe67c4bbda0ba5d4fb5f)) by [@welpo](https://github.com/welpo)
- *(release)* Improve script robustness ([708f907](https://github.com/welpo/git-sumi/commit/708f90738b7d478f0f7da3bd15b264b1f048423a)) by [@welpo](https://github.com/welpo)
- Fix clippy warnings ([d4417a7](https://github.com/welpo/git-sumi/commit/d4417a73f88959984d140057aedc12a04a288013)) by [@welpo](https://github.com/welpo)
- Fix clippy & fmt errors ([20d5cb2](https://github.com/welpo/git-sumi/commit/20d5cb24b7bfc59e5740bd6f4e08c820cee0d449)) by [@welpo](https://github.com/welpo)
- Update git-cliff variables to `commit.remote` ([3c7b486](https://github.com/welpo/git-sumi/commit/3c7b48622f476a328d601fcbb5c6cdc80db8ee93)) by [@welpo](https://github.com/welpo)
- Add funding information ([362fc3c](https://github.com/welpo/git-sumi/commit/362fc3cc71ce82cfb57ef848d59210ec81414e4f)) by [@welpo](https://github.com/welpo)

### 👥 New contributors

🫶 [@TribuneX](https://github.com/TribuneX) made their first contribution in [#199](https://github.com/welpo/git-sumi/pull/199)

## [0.0.9](https://github.com/welpo/git-sumi/compare/v0.0.8..v0.0.9) - 2024-05-06

### ✨ Features

- *(imperative)* Expand non-imperative verbs list ([dfde2a9](https://github.com/welpo/git-sumi/commit/dfde2a92342daf9d39ef1479fd88729673349935)) by [@welpo](https://github.com/welpo)
- *(imperative)* Expand non-imperative verbs list ([d13e6e5](https://github.com/welpo/git-sumi/commit/d13e6e5a5102972439bd0ceef21da28e925e9a46)) by [@welpo](https://github.com/welpo)

### 📝 Documentation

- *(README)* Update demo ([934cacd](https://github.com/welpo/git-sumi/commit/934cacd886b310b94bc94825035fa5ccb43913b5)) by [@welpo](https://github.com/welpo)
- *(README)* Update demo & add assets ([f65154a](https://github.com/welpo/git-sumi/commit/f65154a276617df03290a7b7097a108981c30e12)) by [@welpo](https://github.com/welpo)
- *(README)* Update demo ([c38d1e0](https://github.com/welpo/git-sumi/commit/c38d1e04b872632c72d1a0e20fa546e570f8d3e7)) by [@welpo](https://github.com/welpo)
- *(README)* Update demo with new hooks call ([6f0db41](https://github.com/welpo/git-sumi/commit/6f0db4188cb82f966d3b45bbf7afc9370fe29333)) by [@welpo](https://github.com/welpo)
- *(website)* Link commit parts FAQ in Rules ([f5c7074](https://github.com/welpo/git-sumi/commit/f5c70747a251949f2114b3981d2b7d356fe088b9)) by [@welpo](https://github.com/welpo)
- *(website)* Update rule comments on examples ([7dd8add](https://github.com/welpo/git-sumi/commit/7dd8add0b9b115c40e67f00eb8536954d87c5b63)) by [@welpo](https://github.com/welpo)

### ♻️ Refactor

- Use fmt::Display for footer ([23f0a3d](https://github.com/welpo/git-sumi/commit/23f0a3dcb5bb3bc6a1a604377e059ec65a75ac73)) by [@welpo](https://github.com/welpo)

### 🔧 Miscellaneous tasks

- *(CD)* Bump git-sumi in git-sumi-action on release ([291e96a](https://github.com/welpo/git-sumi/commit/291e96a1dd2b00b40b36c5ba323bf432bb2fe3b3)) by [@welpo](https://github.com/welpo)
- *(CI)* Upgrade cargo-dist to 0.14.0 ([e3a134f](https://github.com/welpo/git-sumi/commit/e3a134f6ae11c17c784f906a80f605de4d41bbe4)) by [@welpo](https://github.com/welpo)
- *(CI)* Upgrade cargo-dist to 0.11.1 ([726f19c](https://github.com/welpo/git-sumi/commit/726f19c3683143112ecbbc0a9b9befd6c7396afd)) by [@welpo](https://github.com/welpo)
- *(git-sumi)* Improve emoji matching ([ac5fc98](https://github.com/welpo/git-sumi/commit/ac5fc98445a1ba2fcf2054b6fe50e063e070dcab)) by [@welpo](https://github.com/welpo)
- *(git-sumi)* Require a space after the gitmoji ([1456cbc](https://github.com/welpo/git-sumi/commit/1456cbc6fd35bb2135e855638b5ea4b8901126d3)) by [@welpo](https://github.com/welpo)
- *(release)* Replace both pull and issue links ([14979a9](https://github.com/welpo/git-sumi/commit/14979a947c717874c8116bba9b2280c28f290fea)) by [@welpo](https://github.com/welpo)
- *(release)* Update CHANGELOG format ([a7eeb99](https://github.com/welpo/git-sumi/commit/a7eeb99ee867ba15c70863f03b76ef21e08f7d7e)) by [@welpo](https://github.com/welpo)
- Pin dependencies ([2fa284f](https://github.com/welpo/git-sumi/commit/2fa284f99391353600d32a610fa5005768f42a97)) by [@welpo](https://github.com/welpo)

## [0.0.8](https://github.com/welpo/git-sumi/compare/v0.0.7..v0.0.8) - 2024-02-09

### 💥 BREAKING CHANGES 💥

- *(init)* Add prepare-commit-msg hook ([#13](https://github.com/welpo/git-sumi/issues/13)) by [@welpo](https://github.com/welpo)

### ✨ Features

- *(imperative)* Add a few non-imperative verbs ([0f0a867](https://github.com/welpo/git-sumi/commit/0f0a86779f7557b20eee23c51725738c41059219)) by [@welpo](https://github.com/welpo)
- *(init)* [**‼️BREAKING‼️**] Add prepare-commit-msg hook ([#13](https://github.com/welpo/git-sumi/issues/13)) by [@welpo](https://github.com/welpo)

### 🐛 Bug fixes

- Ignore comment lines in commit messages ([#14](https://github.com/welpo/git-sumi/issues/14)) by [@welpo](https://github.com/welpo)

### 📝 Documentation

- *(README)* Add git-sumi badge ([9041419](https://github.com/welpo/git-sumi/commit/90414197c8cd77b30de0ac01965c0a30992d64a1)) by [@welpo](https://github.com/welpo)
- *(README)* Update demo with new default config ([e9ffd54](https://github.com/welpo/git-sumi/commit/e9ffd544d00ff12ac9a4682013a4b5a77a8532fb)) by [@welpo](https://github.com/welpo)

## [0.0.7](https://github.com/welpo/git-sumi/compare/v0.0.6..v0.0.7) - 2024-02-08

### 📝 Documentation

- *(faq)* Set hook to exit early outside main branch ([92d9071](https://github.com/welpo/git-sumi/commit/92d90715e15dcf0cd9470462404c37c5414aa648)) by [@welpo](https://github.com/welpo)
- *(website)* Add pre-built binaries download link ([0d3adf6](https://github.com/welpo/git-sumi/commit/0d3adf67a3ea8d50ab6751107700bc1123b1c9e7)) by [@welpo](https://github.com/welpo)
- Finish renaming yaml -> yml ([4b09485](https://github.com/welpo/git-sumi/commit/4b0948549296fa4e822d7c1df619b0b800a4b247)) by [@welpo](https://github.com/welpo)

### ♻️ Refactor

- Unify and rephrase options descriptions ([#12](https://github.com/welpo/git-sumi/issues/12)) by [@welpo](https://github.com/welpo)
- Rename 'yaml' extensions to 'yml' ([e13808f](https://github.com/welpo/git-sumi/commit/e13808f4529edc71610712f22b1773dde0d22323)) by [@welpo](https://github.com/welpo)

### ✅ Testing

- *(config)* Cover --init overwrite prompts ([3562f23](https://github.com/welpo/git-sumi/commit/3562f23b4307cb7f46f0c61a47d4bc8c1c88d31a)) by [@welpo](https://github.com/welpo)

### 🔧 Miscellaneous tasks

- *(CHANGELOG)* Improve emoji pattern ([a80d2db](https://github.com/welpo/git-sumi/commit/a80d2db46b76b556232d7ddbb084487d9f693ad5)) by [@welpo](https://github.com/welpo)
- *(CI)* Remove manual publish steps ([49b016b](https://github.com/welpo/git-sumi/commit/49b016b7186c4fa1a75cf47c7d33a873a2b9ae1c)) by [@welpo](https://github.com/welpo)
- *(pypi)* Match description to website's ([931386d](https://github.com/welpo/git-sumi/commit/931386d44e5d60ba65aa24e80e883ce2e31983bb)) by [@welpo](https://github.com/welpo)
- *(release)* Verify version tag format on release ([8eca1e3](https://github.com/welpo/git-sumi/commit/8eca1e32e1e098cc9ee31554366ebc9edb1b1395)) by [@welpo](https://github.com/welpo)
- Add pre-commit & commit-msg scripts ([2f5ff91](https://github.com/welpo/git-sumi/commit/2f5ff9155cdc28c073c4336d82033381003be04f)) by [@welpo](https://github.com/welpo)

## [0.0.6](https://github.com/welpo/git-sumi/compare/v0.0.5..v0.0.6) - 2024-02-05

### 🔧 Miscellaneous tasks

- *(Cargo.toml)* Exclude more unneeded directories ([76b21c4](https://github.com/welpo/git-sumi/commit/76b21c44231ba3f26bac9806bc6ab589df05f71f)) by [@welpo](https://github.com/welpo)
- *(release)* Upload and download PyPi wheels ([16b9e47](https://github.com/welpo/git-sumi/commit/16b9e47817a02ccb1ec2cdbcefbd9ba04bb3984f)) by [@welpo](https://github.com/welpo)

## [0.0.5](https://github.com/welpo/git-sumi/compare/v0.0.4..v0.0.5) - 2024-02-05

### 📝 Documentation

- Add Deploy badge ([e339cdb](https://github.com/welpo/git-sumi/commit/e339cdbf6dff791b3812af5ec55841da8606a490)) by [@welpo](https://github.com/welpo)
- Add GitHub Releases badge ([2bd1d0f](https://github.com/welpo/git-sumi/commit/2bd1d0fce9632b77f48458f308e67e93357acf2f)) by [@welpo](https://github.com/welpo)
- Remove references to `capitalize_description` ([c2a3044](https://github.com/welpo/git-sumi/commit/c2a3044b985c75924b47c1cede8d76205837abbb)) by [@welpo](https://github.com/welpo)

### 🔧 Miscellaneous tasks

- *(CI)* Add labels to renovate's PRs ([16ba844](https://github.com/welpo/git-sumi/commit/16ba8448c82710781d3fdb932f2ae208294e0405)) by [@welpo](https://github.com/welpo)
- *(CI)* Use single quotes for cron schedule ([5328eca](https://github.com/welpo/git-sumi/commit/5328eca7e3e934bb7ee98d8d34be3b4d9a427ef1)) by [@welpo](https://github.com/welpo)
- *(Cargo.toml)* Exclude unnecessary directories ([cc7d3a7](https://github.com/welpo/git-sumi/commit/cc7d3a7c798f65313c739ec4e42f267860bd8ca4)) by [@welpo](https://github.com/welpo)
- *(release)* Fix cargo publish ([65cfcc9](https://github.com/welpo/git-sumi/commit/65cfcc987205a92e8a5c5af96967a68a387bffd4)) by [@welpo](https://github.com/welpo)
- *(release)* Split maturin build/publish processes ([b798857](https://github.com/welpo/git-sumi/commit/b79885759170ccf88c6ff3d05f08fac54863369a)) by [@welpo](https://github.com/welpo)

## [0.0.4](https://github.com/welpo/git-sumi/compare/v0.0.3..v0.0.4) - 2024-02-05

### ✨ Features

- *(config)* Override config bools with env vars ([#9](https://github.com/welpo/git-sumi/issues/9)) by [@welpo](https://github.com/welpo)
- *(imperative)* Expand non-imperative verbs ([22033cc](https://github.com/welpo/git-sumi/commit/22033cced1bc0eaaa85d6b7271eeada00d30f6fa)) by [@welpo](https://github.com/welpo)
- *(imperative)* Expand non-imperative verbs ([2f7dcba](https://github.com/welpo/git-sumi/commit/2f7dcba50033f17761c042d51c63fccfb0755e73)) by [@welpo](https://github.com/welpo)

### 🐛 Bug fixes

- *(init)* Only set permissions on Unix ([feeb22a](https://github.com/welpo/git-sumi/commit/feeb22ab96652712f71c9124ca5119035b90ab65)) by [@welpo](https://github.com/welpo)

### 📝 Documentation

- *(website)* Fix git-sumi installed check ([c4244b9](https://github.com/welpo/git-sumi/commit/c4244b9be7a2b68a4b5c3d4e8747a12e839b984e)) by [@welpo](https://github.com/welpo)

### 🔧 Miscellaneous tasks

- *(CD)* Use cargo-dist for releases ([8a376a4](https://github.com/welpo/git-sumi/commit/8a376a40810654a4ef86f5e14f5278b1c9830ed6)) by [@welpo](https://github.com/welpo)
- *(CHANGELOG)* Update grouping ([cc12a84](https://github.com/welpo/git-sumi/commit/cc12a840d82014a4ee8e9cca988a2358c111c139)) by [@welpo](https://github.com/welpo)
- *(CI)* Exclude repo's commits from link checking ([e7b58b3](https://github.com/welpo/git-sumi/commit/e7b58b301ec6e62b26b208e63ddba8d6ec308f49)) by [@welpo](https://github.com/welpo)
- *(CI)* Rename git-sumi job ([e9ba384](https://github.com/welpo/git-sumi/commit/e9ba384756dff8619861ca87167160a587b5b3d5)) by [@welpo](https://github.com/welpo)
- *(release)* Remove `--frozen` from maturin command ([0a08d30](https://github.com/welpo/git-sumi/commit/0a08d3048905e848b0b43dc72b765235c11f2b47)) by [@welpo](https://github.com/welpo)
- *(release)* Check for clean working directory ([ebfc43a](https://github.com/welpo/git-sumi/commit/ebfc43a72555d209e47533b0249dba44b0d01d2e)) by [@welpo](https://github.com/welpo)

## [0.0.3](https://github.com/welpo/git-sumi/compare/v0.0.2..v0.0.3) - 2024-02-05

### 🐛 Bug fixes

- *(basic-parser)* Capture original header ([ba3b413](https://github.com/welpo/git-sumi/commit/ba3b4137dce7c02988f45c45d873c4d747cd09c4)) by [@welpo](https://github.com/welpo)

### 🔧 Miscellaneous tasks

- *(CI)* Fix links in tag description ([8f0702d](https://github.com/welpo/git-sumi/commit/8f0702df3a49c78f481aff9953fdb7e27d39c4f6)) by [@welpo](https://github.com/welpo)
- *(README)* Increase spacing below logo ([fb91e20](https://github.com/welpo/git-sumi/commit/fb91e20acf3018ffddd23c81b10724d0e65bd8b6)) by [@welpo](https://github.com/welpo)
- *(release)* Fix sed command ([49f3f16](https://github.com/welpo/git-sumi/commit/49f3f1601cf5057ae92b03818556c6a381d74e87)) by [@welpo](https://github.com/welpo)
- Fix Cargo.lock ([cfa252d](https://github.com/welpo/git-sumi/commit/cfa252d964d5369e424e0d9ba5ffce2f47b989f8)) by [@welpo](https://github.com/welpo)

## [0.0.2](https://github.com/welpo/git-sumi/compare/v0.0.1..v0.0.2) - 2024-02-04

### 💥 BREAKING CHANGES 💥

- *(display)* Add markdown table format ([#7](https://github.com/welpo/git-sumi/issues/7)) by [@welpo](https://github.com/welpo)

### ✨ Features

- *(display)* [**‼️BREAKING‼️**] Add markdown table format ([#7](https://github.com/welpo/git-sumi/issues/7)) by [@welpo](https://github.com/welpo)

### 🔧 Miscellaneous tasks

- *(release)* Auto-update version in Cargo.toml ([312e12c](https://github.com/welpo/git-sumi/commit/312e12c69810ad49542d265dc982adaf487b209b)) by [@welpo](https://github.com/welpo)

## 0.0.1 - 2024-02-04

### ✨ Features

- Initial commit ([368b7e7](https://github.com/welpo/git-sumi/commit/368b7e72628a26e818d4a20d324d0d6715c7f7e9)) by [@welpo](https://github.com/welpo)

### 📝 Documentation

- *(README)* Add CI badge ([8f1a909](https://github.com/welpo/git-sumi/commit/8f1a909477222d1bdb51c25624327d269977706f)) by [@welpo](https://github.com/welpo)

### ✅ Testing

- Fix clippy warnings ([b21b5ca](https://github.com/welpo/git-sumi/commit/b21b5ca265e5b784e1d02e51fb20d37dd2853694)) by [@welpo](https://github.com/welpo)
- Disable GPG signing ([c736369](https://github.com/welpo/git-sumi/commit/c736369e170dd1522b63e4d4bc349b513e398c22)) by [@welpo](https://github.com/welpo)

### 🔧 Miscellaneous tasks

- *(CI)* Use LLVM engine for test coverage ([79bdd87](https://github.com/welpo/git-sumi/commit/79bdd8771341a151fa3be081957b8149a5d9a661)) by [@welpo](https://github.com/welpo)
- *(CI)* Remove redundant step ([04a0e48](https://github.com/welpo/git-sumi/commit/04a0e488d518984fe17b6bb4dd932a2ea77e9a82)) by [@welpo](https://github.com/welpo)
- *(CI)* Change tarpaulin install source ([90841c9](https://github.com/welpo/git-sumi/commit/90841c933035dfb465424884a22aaac0644c5e03)) by [@welpo](https://github.com/welpo)
- *(Cargo.toml)* Update keywords ([06d9770](https://github.com/welpo/git-sumi/commit/06d9770f8a985df2f21d5c9b93f8bbbc1ee7025e)) by [@welpo](https://github.com/welpo)
- *(README)* Add crates.io shield ([08197cb](https://github.com/welpo/git-sumi/commit/08197cba0fabdb0f5c3f9c7673d6214f67e20fbc)) by [@welpo](https://github.com/welpo)
- *(release)* Display command to publish on cargo ([3f05aba](https://github.com/welpo/git-sumi/commit/3f05abadde7cd2bd6542b3ac1ca85ef60b3a7b6f)) by [@welpo](https://github.com/welpo)
- *(renovate)* Move config file ([8d7f116](https://github.com/welpo/git-sumi/commit/8d7f116cd7bb8c3c62963f8e1c9f1ef2b1061702)) by [@welpo](https://github.com/welpo)

### 👥 New contributors

🫶 [@welpo](https://github.com/welpo) made their first contribution

🫶 [@renovate](https://www.mend.io/renovate/)[bot] made their first contribution in [#3](https://github.com/welpo/git-sumi/pull/3)

<!-- generated by git-cliff -->

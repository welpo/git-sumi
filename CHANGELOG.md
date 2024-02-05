# Changelog

Welcome to the changelog for git-sumi. This document aims to provide a comprehensive list of all notable changes made to the project, organised chronologically by release version.

We use Semantic Versioning (SemVer) for our version numbers, formatted as MAJOR.MINOR.PATCH. Major version changes involve significant (breaking) changes, minor versions introduce features and improvements in a backward compatible manner, and patch versions are for bug fixes and minor tweaks.

## [0.0.4](https://github.com/welpo/git-sumi/compare/v0.0.3..v0.0.4) - 2024-02-05

### ✨ Features

- *(config)* Override config bools with env vars ([#9](https://github.com/welpo/git-sumi/issues/9)) - ([e84a27e](https://github.com/welpo/git-sumi/commit/e84a27eb892384fe7e43e41ae832901dae50ba93))
- *(imperative)* Expand non-imperative verbs - ([22033cc](https://github.com/welpo/git-sumi/commit/22033cced1bc0eaaa85d6b7271eeada00d30f6fa))
- *(imperative)* Expand non-imperative verbs - ([2f7dcba](https://github.com/welpo/git-sumi/commit/2f7dcba50033f17761c042d51c63fccfb0755e73))

### 🐛 Bug fixes

- *(init)* Only set permissions on Unix - ([feeb22a](https://github.com/welpo/git-sumi/commit/feeb22ab96652712f71c9124ca5119035b90ab65))

### 📝 Documentation

- *(website)* Fix git-sumi installed check - ([c4244b9](https://github.com/welpo/git-sumi/commit/c4244b9be7a2b68a4b5c3d4e8747a12e839b984e))

### 🔧 Miscellaneous tasks

- *(CD)* Use cargo-dist for releases - ([8a376a4](https://github.com/welpo/git-sumi/commit/8a376a40810654a4ef86f5e14f5278b1c9830ed6))
- *(CHANGELOG)* Update grouping - ([cc12a84](https://github.com/welpo/git-sumi/commit/cc12a840d82014a4ee8e9cca988a2358c111c139))
- *(CI)* Exclude repo's commits from link checking - ([e7b58b3](https://github.com/welpo/git-sumi/commit/e7b58b301ec6e62b26b208e63ddba8d6ec308f49))
- *(CI)* Rename git-sumi job - ([e9ba384](https://github.com/welpo/git-sumi/commit/e9ba384756dff8619861ca87167160a587b5b3d5))
- *(release)* Check for clean working directory - ([ebfc43a](https://github.com/welpo/git-sumi/commit/ebfc43a72555d209e47533b0249dba44b0d01d2e))

## [0.0.3](https://github.com/welpo/git-sumi/compare/v0.0.2..v0.0.3) - 2024-02-05

### 🐛 Bug fixes

- *(basic-parser)* Capture original header - ([ba3b413](https://github.com/welpo/git-sumi/commit/ba3b4137dce7c02988f45c45d873c4d747cd09c4))

### 🔧 Miscellaneous tasks

- *(CI)* Fix links in tag description - ([8f0702d](https://github.com/welpo/git-sumi/commit/8f0702df3a49c78f481aff9953fdb7e27d39c4f6))
- *(README)* Increase spacing below logo - ([fb91e20](https://github.com/welpo/git-sumi/commit/fb91e20acf3018ffddd23c81b10724d0e65bd8b6))
- *(release)* Fix sed command - ([49f3f16](https://github.com/welpo/git-sumi/commit/49f3f1601cf5057ae92b03818556c6a381d74e87))
- Fix Cargo.lock - ([cfa252d](https://github.com/welpo/git-sumi/commit/cfa252d964d5369e424e0d9ba5ffce2f47b989f8))

## [0.0.2](https://github.com/welpo/git-sumi/compare/v0.0.1..v0.0.2) - 2024-02-04

### 💥 BREAKING CHANGES 💥

- *(display)* Add markdown table format ([#7](https://github.com/welpo/git-sumi/issues/7)) - ([2098e3b](https://github.com/welpo/git-sumi/commit/2098e3bea0c3a87a3ef477f7da68cd0cb275790c))

### ✨ Features

- *(display)* [**‼️BREAKING‼️**] Add markdown table format ([#7](https://github.com/welpo/git-sumi/issues/7)) - ([2098e3b](https://github.com/welpo/git-sumi/commit/2098e3bea0c3a87a3ef477f7da68cd0cb275790c))

### 🔧 Miscellaneous tasks

- *(release)* Auto-update version in Cargo.toml - ([312e12c](https://github.com/welpo/git-sumi/commit/312e12c69810ad49542d265dc982adaf487b209b))

## 0.0.1 - 2024-02-04

### ✨ Features

- Initial commit - ([368b7e7](https://github.com/welpo/git-sumi/commit/368b7e72628a26e818d4a20d324d0d6715c7f7e9))

### 📝 Documentation

- *(README)* Add CI badge - ([8f1a909](https://github.com/welpo/git-sumi/commit/8f1a909477222d1bdb51c25624327d269977706f))

### ✅ Testing

- Fix clippy warnings - ([b21b5ca](https://github.com/welpo/git-sumi/commit/b21b5ca265e5b784e1d02e51fb20d37dd2853694))
- Disable GPG signing - ([c736369](https://github.com/welpo/git-sumi/commit/c736369e170dd1522b63e4d4bc349b513e398c22))

### 🔧 Miscellaneous tasks

- *(CI)* Use LLVM engine for test coverage - ([79bdd87](https://github.com/welpo/git-sumi/commit/79bdd8771341a151fa3be081957b8149a5d9a661))
- *(CI)* Remove redundant step - ([04a0e48](https://github.com/welpo/git-sumi/commit/04a0e488d518984fe17b6bb4dd932a2ea77e9a82))
- *(CI)* Change tarpaulin install source - ([90841c9](https://github.com/welpo/git-sumi/commit/90841c933035dfb465424884a22aaac0644c5e03))
- *(Cargo.toml)* Update keywords - ([06d9770](https://github.com/welpo/git-sumi/commit/06d9770f8a985df2f21d5c9b93f8bbbc1ee7025e))
- *(README)* Add crates.io shield - ([08197cb](https://github.com/welpo/git-sumi/commit/08197cba0fabdb0f5c3f9c7673d6214f67e20fbc))
- *(release)* Display command to publish on cargo - ([3f05aba](https://github.com/welpo/git-sumi/commit/3f05abadde7cd2bd6542b3ac1ca85ef60b3a7b6f))
- *(renovate)* Move config file - ([8d7f116](https://github.com/welpo/git-sumi/commit/8d7f116cd7bb8c3c62963f8e1c9f1ef2b1061702))

<!-- generated by git-cliff -->

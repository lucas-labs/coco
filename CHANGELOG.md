# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.1] - 2024-09-28

### ✨ Features
- Main committing logic by [@lucas-labs](https://github.com/lucas-labs) [`7f4ce1e`](7f4ce1ee5829f7c4d91756dc925ddeb2b4f936cd)
- Builder type step header by [@lucas-labs](https://github.com/lucas-labs) [`b64c5ad`](b64c5ad92cf295c752e176b96107478658d5ab9a)
- Add headers to all steps by [@lucas-labs](https://github.com/lucas-labs) [`90c14b9`](90c14b9d13fbd592887e70f50ed1744df6adcbec)
- Show the final commit message before quitting by [@lucas-labs](https://github.com/lucas-labs) [`PR #8`](https://github.com/lucas-labs/coco/pull/8)
- Use theme colors instead of hardcoded colors by [@lucas-labs](https://github.com/lucas-labs) [`PR #9`](https://github.com/lucas-labs/coco/pull/9)
- Add real key binding glossary in the help view by [@lucas-labs](https://github.com/lucas-labs) [`PR #10`](https://github.com/lucas-labs/coco/pull/10)
- Conditional form steps by [@lucas-labs](https://github.com/lucas-labs) [`ed822de`](ed822de778bc0c0c749388b3c2fc9386bbd3ba20)
- Conditional builder steps for commit message textareas by [@lucas-labs](https://github.com/lucas-labs) [`PR #11`](https://github.com/lucas-labs/coco/pull/11)
- Calculate the real max char count for the summary by [@lucas-labs](https://github.com/lucas-labs) [`PR #17`](https://github.com/lucas-labs/coco/pull/17)
- Ability to set the locale via env var on debug builds by [@lucas-labs](https://github.com/lucas-labs) [`6a4f666`](6a4f6663a84872abcf1274868e4e2f39d9b3a5dc)
- Don't show the commit message if the user quitted prematurely by [@lucas-labs](https://github.com/lucas-labs) [`PR #22`](https://github.com/lucas-labs/coco/pull/22)

### 🚑 Bug Fixes
- Fix english translation for the "creating commit" message by [@lucas-labs](https://github.com/lucas-labs) [`PR #18`](https://github.com/lucas-labs/coco/pull/18)
- Hardcoded commit info in summary section by [@lucas-labs](https://github.com/lucas-labs) [`4c0059f`](4c0059fb69f3c149bcca7c8fb0eafc5cb16ca1e5)

### 🔨 Refactor
- Restructure the project to make it a single crate by [@lucas-labs](https://github.com/lucas-labs) [`PR #25`](https://github.com/lucas-labs/coco/pull/25)
- Remove lool dependency by [@lucas-labs](https://github.com/lucas-labs) [`787f196`](787f1965bdb2121796e4f2437a175a33ec719e54)

### 📝 Documentation
- Add README.md file by [@lucas-labs](https://github.com/lucas-labs) [`PR #23`](https://github.com/lucas-labs/coco/pull/23)

### 🌐 Internationalization
- Implement i18n using rust-i18n crate by [@lucas-labs](https://github.com/lucas-labs) [`d49236b`](d49236b3277f545c18fd21cc3a502cb8cccbcd69)
- Summary view i18n translation by [@lucas-labs](https://github.com/lucas-labs) [`PR #21`](https://github.com/lucas-labs/coco/pull/21)

### ⚙️  Miscellaneous Tasks
- Implement --no-stage-check argument by [@lucas-labs](https://github.com/lucas-labs) [`1d74e05`](1d74e057f2481fb1b72858524eec000b6e8cea11)
- Prepare the package to be published on crates.io by [@lucas-labs](https://github.com/lucas-labs) [`PR #26`](https://github.com/lucas-labs/coco/pull/26)
- Add cargo dist configuration by [@lucas-labs](https://github.com/lucas-labs) [`ad31d09`](ad31d096ea1383357089e698e86cd194653e4254)
- Configure cargo-release and cliff by [@lucas-labs](https://github.com/lucas-labs) [`3586288`](358628833c61c6e1ca0a6d71d25c56f2e85b255a)
- Release tasks by [@lucas-labs](https://github.com/lucas-labs) [`b48685c`](b48685c61f0aa57fe7543ce1951cbbbb780ca00e)
- Changelog task by [@lucas-labs](https://github.com/lucas-labs) [`abf23da`](abf23da0cbd31757c4b969455a6541144cc2dea4)
- Remove repeated keyword in Cargo.toml by [@lucas-labs](https://github.com/lucas-labs) [`3dfdc8b`](3dfdc8be85a4cfac2be967249898737a097f0d72)

## New Contributors
* [@lucas-labs](https://github.com/lucas-labs) made their first contribution 🎉


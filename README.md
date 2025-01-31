# rust-template

## CI & deployment

This repo includes the following CI:
- cargo fmt (blocking)
- cargo check
- cargo clippy (blocking)
- cargo test (for linux, macos and windows)
- cargo coverage (will post the result as an (auto-updated) PR comment)
- cargo docs generation

- cargo audit (checking for imported crates with known vulnerabilities)

This repo follows the standard Wonderland repo structure:
- feat, fix and test branches, which are merged into dev once feature comlete
- dev contains all the active development code which has matured
- main contains all the released versions

The CI will deploy on both crates.io and as a github release all push on main which
are tagged with a valid semver preceded by `v` (`git tag -a vX.Y.Z -m "Release X.Y.Z"` then `git push vX.Y.Z` on dev).
This deployment needs an env secret `CRATES_API_TOKEN` to be set in the repo settings as well as a "prod" environement, 
with a protection rule on the branch `main` and on the `v*.*.*` pattern tag.

![image](https://github.com/user-attachments/assets/72e63e2e-10f2-40cf-a9ae-db7ad5bc2b74)

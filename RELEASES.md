
## Preparing and Publishing Releases
WalletD versioning adheres to [semantic versioning][semver].
### Pre-requisites

- Before pushing your local branch, ensure that all tests pass by running `cargo test` for the specific package you are editting. 
- We have a number of automated CI/CD processes in place for our libraries. Pull requests that don't pass these automated checks will need to be revised to meet all CI/CD checks in place before a maintainer will review them.

### Minimum supported Rust version

 ** KB Note: Presently, this version is up for discussion: Version 1.58.1

## Build methodology

#### Merge new features

TBD: 

Merge in any oustanding pull requests identified for the next release into the `main` branch.
`main` is the nightly branch, and will be built daily. This branch will be bleeding-edge, and may contain bugs.

Create a branch off `main`. Name the new version according to semantic versioning. For example, if the current version if v0.1.0 and you're adding a patch,`git checkout main; git checkout release-v0.1.1`
Increment version numbers in the source code as appropriate
Push the new branch to Github
Make sure all CI/CD passes successfully
Tag the release on GitHub
Publish the new crate(s) to crates.io

The release will be tagged on GitHub, and the various packages will be automatically uploaded to [npm](https://www.npmjs.org)

TBD: release of bindings for various languages and how we handle that

[semver]: https://semver.org
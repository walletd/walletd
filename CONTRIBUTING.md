# WalletD contribution guide

## Effective documentation of our processes

Given the asynchronous nature of open source software development, we believe strongly in documenting our processes, our reasoning and any decisions that may arise in a transparent, collaborative manner.

To this end, when you do discuss aspects of projects within the WalletD ecosystem, please try do so in a manner that allows other contributors and interested parties to see what was discussed, rather than keeping conversations siloed.

Our team may have meetings where ideas and decisions are made. In order to keep this transparent, where possible, we will endeavour to publish a document that summarises all salient points.

The WalletD team strongly believe that good documentation is critical in clarifying a contributor's reasoning and expectations. Ideally all projects should have documentation that is up-to-date, and when undergoing changes, should at least stipulate in the documentation what aspects of it are out of date.

## Contributing to the project
Before any code is written, an issue should be opened detailing a requested feature or a bug. Maintainers will review this issue and tag it accordingly.

Pull requests that do not have an associated issue are far more likely to be closed immediately, if what they address isn't absolutely clear to maintainers. If you aren't fixing typos or adding clarifying code comments only, there's an incredibly high chance that your PR will be rejected and we'll point you to these contribution rules.

The issues raised will be discussed, and should a decision be made to incorporate a fix or a feature request, the responsible contributor should create their own fork of the current version of main. Should you desire some assistance or input while you work on your changes, we recommend you open a draft pull request on the appropriate crate's branch and mark your PR as a work-in-progress as part of the label, as well as by using GitHub's "mark as draft" functionality. A maintainer or other collaborator will then have context to assist where you may be having problems.

In order to contribute to the project, a good place to start is by looking at the issues tab. We will review any issues, tagging them based on their complexity, whether we would like assistance from certain types of contributors, and in the case of new features, discuss whether or not they would be a good fit for the project.

We strongly recommend that in the instance of issues you decide to assist with, reach out on that issue to ensure that it isn't already being worked on, and indicate that you wish to work on that specific issue. This helps avoid duplication of work. 

Remember, your options for contribution to the project need not necessarily just be features. Code comments explaining the nuances of the code in a way a layperson can understand are also incredibly helpful for any newcomers to the project. Any additional assistance by adding examples, documentation, testing and answering questions other contributors have are all contributions that we truly welcome.

## Pull request guidelines

### What makes a pull request good?
Due to the collaborative nature of contributing to open source projects, it's difficult to efficiently convey what you were thinking when you wrote your code.

There are ways to address this. One thing WalletD does whenever possible is use some form of [Conventional Commits][conventional-commits], which defines a number of different types of commits. **Generally, if you find you need to use two types when creating a git commit message, your commit almost certainly could be broken into two or more separate commits**.

We suggest contributors review at least the quick summary of this standard before they start coding to assist others in understanding how the changes they make affect the codebase. 

Without going into too much detail regarding Conventional Commits, one should make one pull request per "type" of your changes. An acceptable commit prefix message could be: `docs: updated typo`. 

For more intricate commits, we suggest adding a body to your commit messages explaining the changes made by providing more context than a simple commit message. 

An acceptable example commit message that follows this can take the structure that follows: 
```
fix: fixes bug related to issue 1173

Issue 1173 caused a problem with the handling of ...
This commit fixes ... by changing ... to address ...

[optional footer(s)]
```
WalletD maintainers generally tend to use one of the following types for each commit (this is non-exhaustive): 

build
chore
ci
docs
feat*
fix*
style
refactor
perf
test


 *"fix" and "feat" commits are special cases where the commit should necessitate a new version of the repository to be released. Fix corresponds to PATCH and feat corresponds to MINOR in [Semantic Versioning][semver].

Commits that have breaking changes correspond to a MAJOR semver change, and can be part of any type of tag (feat, build, etc). 

Any breaking change commit should have a footer that says BREAKING CHANGE:, or append an ! after the type/scope. A commit of this nature introduces a non-backwards-compatible change (correlating with MAJOR in Semantic Versioning). 

A BREAKING CHANGE can be part of commits of any type.
### How to contribute
To get started, fork the current main branch and work from there. 

1. Please restrict your pull request(s) to one feature per branch
1. Ensure that your pull request does not cause a regression error by running `cargo test` in your CLI
1. Avoid pull requests that change large numbers of files at one time. By avoiding sweeping changes, this will allow reviewers to properly review your changes and follow up with you.
1. Commits that touch too many files at once may have maintainers requesting you to reduce the scope of the changes you have made to allow us to better understand your changes and their effects.

### How to contribute your own commits
New to open source? We have a first-time guide tailored to help you as part of this document, and suggest you review that if you're not yet comfortable contributing to open source projects.

You are welcome to contact us via GitHub issues should you have any questions. If you haven't heard from us in seven days, please feel free to ping us in discussions/issues to remind us that we need to get back to you.



## Writing your first-ever open source contributions
Commiting to a new open source project for the first time can be intimidating, but it doesn't have to be. It's a process that can be easily summarised into a few short points.

1. Fork the repo on GitHub.
1. Clone your fork to your development computer
```
git clone git@github.com:<yournamehere>/mnemonic.git
```
3. Once you've made the changes you'd like, run the test suite using the following command:
```
cargo test
```
4. Make sure all tests pass. If you've done some work and are stuck, you're welcome to reach out to us via GitHub issues, linking the branch and fork you're working on.
4. If everything passes testing, push your changes to your fork
4. Open a pull request on this project, targeting the `main` branch with your changes.
4. Please be patient with feedback. We're a small team who maintain a large collection of software, so we may not always get to you as promptly as we would like to.


If you're not planning on contributing code or documentation changes, please restrict your interaction to issues and discussions.

If you are planning on adding features, documentation or bug fixes, that's really appreciated as well.

[conventional-commits]: https://www.conventionalcommits.org/en/v1.0.0/
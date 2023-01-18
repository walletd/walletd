# walletd contribution guide

## Effective documentation of our processes

Given the asynchronous nature of open source software development, we believe strongly in documenting our processes, our reasoning and any decisions that may arise in a transparent, collaborative manner.

To this end, when you do discuss aspects of projects within the walletd ecosystem, please try do so in a manner that allows other contributors and interested parties to see what was discussed, rather than keeping conversations siloed.

Our team may have meetings where ideas and decisions are made. In order to keep this transparent, where possible, we will endeavour to publish a document that summarises all salient points **Maybe we do this in a repo where we store minutes?**

The walletd team strongly believe that good documentation is critical in clarifying a contributor's reasoning and expectations. Ideally all projects should have documentation that is up-to-date, and when undergoing changes, should at least stipulate in the documentation what aspects of it are out of date.

## Contributing to the project

In order to contribute to the project, a good place to start is by looking at the issues tab. We will review any issues, tagging them based on their complexity, whether we would like assistance from certain types of contibutors, and in the case of new features, discuss whether or not they would be a good fit for the project.

We strongly recommend that in the instance of issues you decide to tackle, to reach out to indicate that you desire to work on that specific issue, so as to avoid duplication of work. 

Remember, your options for contribution to the project need not necessarily just be features. Code comments explaining the complexities of the code in a way a layperson can understand are incredibly helpful for any newcomers to the project. Also, any assistance with additional examples, documentation, testing and suggested features are also greatly welcomed.

## Pull request guidelines

To get started, fork the current main branch and work from there. 

1. Please restrict your pull request(s) to one feature per branch
1. Ensure that your pull request does not cause a regression error by running `cargo test` in your CLI
1. Avoid pull requests that change large numbers of files at one time. By avoiding sweeping changes,this will allow reviewers to properly review your changes and follow up with you.

### How to contribute your own commits
New to open-source? We have a first-time guide tailored to help you as part of this document, and suggest you review that if you're not yet comfortable contributing to open source projects.

1. Fork the repo on Github
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
4. Please be patient with feedback. We're a small team who maintain a large collection of software, so we may not always get to you. 

If you haven't heard from us in seven days, please feel free to ping us in discussions/issues to remind us that we need to get back to you.

## Writing your first-ever open source contributions
Commiting to a new open source project for the first time can be intimidating, but it doesn't have to be. It's a process that can be easily summarised into a few short points.

If you're not planning on contributing code or documentation changes, please restrict your interaction to issues and discussions.

If you are planning on adding features, documentation or bug fixes, that's really appreciated as well.

For more information regarding the release cycle of this project, please refer to [the documentation][readme-example]


[readme-example]: https://github.com/walletd/hd_key/tree/main/examples/readme
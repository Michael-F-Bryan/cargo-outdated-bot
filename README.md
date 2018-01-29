# Cargo Outdated Bot

A bot for automatically checking every crate on [crates.io] and making sure 
their dependencies are up to date.


## Architecture

There are three main pieces:

### Master Node

Keeps track of the current state of [crates.io] and outstanding/running jobs. 
Each task runner will contact the master node with a list of jobs it supports, 
asking if there are any tasks available.

### Outdated Runner

Downloads a crate and checks what dependencies can be upgraded, making sure the
crate still compiles (`cargo check`). Reports back to the master node with
the results.

### Pull Request Runner

If the crate has a `repository` on GitHub, it will apply the upgrades and then
send in a PR.


[crates.io]: https://crates.io/
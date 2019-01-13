# Ray tracing [![Build Status](https://travis-ci.org/elpiel/ray-tracing.svg?branch=master)](https://travis-ci.org/elpiel/ray-tracing) [![codecov](https://codecov.io/gh/elpiel/ray-tracing/branch/master/graph/badge.svg)](https://codecov.io/gh/elpiel/ray-tracing)


## In a weekend

### Running the project

`cargo run`

### Running the tests

`cargo test`

### Code checks

Apart from the tests, there are couple of checks that are running and they should pass in order
for the PRs to be passing the builds:

#### Cargo format
You can run it by this command:

`cargo fmt -- --check`

If you want to fix the formatting automatically, just drop the check:

`cargo fmt`

#### Cargo Clippy

Cargo clippy is also running, for now with one lint disabled, until I fix it in a appropriate way.
To check the code, please run:

`cargo clippy --all-targets --all-features -- -D warnings -A clippy::float_cmp`

### Future work:

* Use Cargo make
* Add git hooks to repository
* Fix [codecov](https://codecov.io/gh/elpiel/ray-tracing) for the binary.

### TODOs:

* Write tests for the Hitable::hit of a Sphere

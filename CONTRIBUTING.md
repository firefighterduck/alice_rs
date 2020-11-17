# Contributing to Alice_rs

Dear reader, if you read these lines you seem to be interested in contributing to Alice_rs, which is more than ever wished for for this project.
Therefore, have my gratitude and feel free to propose any contributions in accordance with the following guidelines.

## What may help me starting work on this project?

### Theory
Before you start diving in the source code you might want to learn about the theoretical background of this project.
As mentioned in the README, Alice_rs is based on the paper [A Decidable Fragment of Separation Logic](http://www0.cs.ucl.ac.uk/staff/p.ohearn/papers/unroll_collapse_withproofs.pdf]).
This paper describes one of the earliest solvers for separation logic formulæ.
It can also been used as a first introduction to the general topic of separation logic.
A more recent description of the same fragment of separation logic can be found in the [seminar paper](https://www21.in.tum.de/teaching/sar/SS20/8.pdf) this project accompanied.

### Rust
Other than the theoretical background work on this project may require knowledge about the rust programming language to some extent.
This knowledge can be acquired from many sources; yet the recommended is the official [rust lang book](https://doc.rust-lang.org/stable/book/).

## Development Environment
Alice_rs doesn't need any special environmental setup. 
A standard rust language and cargo installation is sufficient.
The build was tested with the current stable versions of these tools on both Windows 10, Ubuntu as WSL and Ubuntu 20.04 LTS.

The project requires no specific IDE, yet Visual Studio Code is recommended.
It is recommended to also use the rust-analyzer extension.

## How can I contribute?
Currently there is no fixed contribution schema.
It is only advised to follow the standard Github workflow (open an issue, fork the project, make a pull request).
You are also free to make bug reports via Github issues, submit changes via Pull Requests or simply fork the repository and make your own version of Alice_rs.
Feel free to contribute in any way you like.

## Styleguides
There is no real style guide used for this project.
As long as the code is formatted with rustfmt (e.g. via the rust-analyzer extension in VS Code) and checked against the clippy linter it should not be rejected for style reasons.
Both come with every installation of the rustc compiler via rustup (see e.g. the Installation guide in the [README](README.md#installation)).

The current internal representation is based on functional programming paradigms but this may also change in the future and is not required to imitate.

Any new contribution should also be thoroughly tested.
The current way of doing this is via unit tests in the same file.
This may change at a later point of time.

## Code of Conduct
By interacting in any way with this project (other than forking) thou shalt comply with the [Code of Conduct](CODE_OF_CONDUCT.md).
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md) ![GitHub](https://img.shields.io/github/license/firefighterduck/alice_rs)

# Alice_rs
Alice_rs is a small proof-of-concept reference implementation of a decision procedure for [A Decidable Fragment of Separation Logic](http://www0.cs.ucl.ac.uk/staff/p.ohearn/papers/unroll_collapse_withproofs.pdf) and was written as supplemental material for [this](https://www21.in.tum.de/teaching/sar/SS20/8.pdf) seminar paper.
The name comes from a wrong pronunciation of the ls structure. The correct pronunciation would be "list structure" but it could also be read as "al-as structure" which sounds a bit like alice.

## Installation
Alice_rs is currently only available through this repository. <br />
It is recommended to use a current version of the rust language compiler with cargo. Both can be obtained from [here](https://rustup.rs/).
The build process is based around the standard cargo build tool chain (it is recommended for non development builds to use the `--release` flag):
```bash
cargo build
```

## Usage 
Alice_rs takes an entailment as a string command line argument like this: `alice_rs "[here goes the entailment]"`.

Example: 
```bash
alice_rs "And[Neq(x,y)]|SepConj[x->y,y->Nil] |- True|SepConj[ls(x, Nil)]"
```
For those unexperienced in this kind of separation logic, here is a short introduction to the semantics:<br />
An entailment describes that for all states (a formal description of a stack and heap architecture) for which the left formula holds the right formula should hold as well.
A formula consists of statements about a state.
These statements are organized in two parts: first the pure logic (reasons about equality) and the spatial logic (reasons about the heap structure).<br />
For more information please have a look at the theoretical background mentioned above ([this](http://www0.cs.ucl.ac.uk/staff/p.ohearn/papers/unroll_collapse_withproofs.pdf) and [this](https://www21.in.tum.de/teaching/sar/SS20/8.pdf)).

The grammar for the entailment strings is based on standard separation logic formulæ (definitions in order of priority):

Nonterminal | | Definition
------- | ---- | --------
*Entailment* | &rarr; | *Formula* \|- *Formula*
*Formula* | &rarr; | *Pure* \| *Spatial*
*Pure* | &rarr; | True
*Pure* | &rarr; | And[*Op_Vec*]
*Op_Vec* | &rarr; | *Op*, *Op_Vec*
*Op_Vec* | &rarr; | *Op*
*Op* | &rarr; | Eq(*Expr*, *Expr*)
*Op* | &rarr; | Neq(*Expr*, *Expr*)
*Spatial* | &rarr; | Emp
*Spatial* | &rarr; | SepConj[*Spatial_Vec*]
*Spatial_Vec* | &rarr; | *AtomicSpatial*, *Spatial_Vec*
*Spatial_Vec* | &rarr; | *AtomicSpatial*
*AtomicSpatial* | &rarr; | *Expr* -> *Expr*
*AtomicSpatial* | &rarr; | ls(*Expr*, *Expr*)
*Expr* | &rarr; | Nil
*Expr* | &rarr; | [a-zA-z]+

To run in the development environment simply use `cargo run [here goes the entailment]` (the `--release` flag can be used with this as well.

Tests can be run with `cargo test`.

## Results
If the program returns nothing, the entailment is valid.
Otherwise either a parser error occurred or the entailment is found invalid. These errors are currently only handled via rust's panic mechanism. A more sophisticated error handling is yet to be implemented.

## Project Status
Despite this project being a complete proof-of-concept implementation further development is planned. Especially the internal representation will be the issue of further improvements.

## Contributing
See the [Contributing](CONTRIBUTING.md) file.

## License
Alice_rs is licensed under the [MIT license](LICENSE).

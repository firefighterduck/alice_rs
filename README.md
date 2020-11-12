# Alice_rs
Alice_rs is a small proof-of-concept reference implementation of a decision procedure for [A Decidable Fragment of Separation Logic](http://www0.cs.ucl.ac.uk/staff/p.ohearn/papers/unroll_collapse_withproofs.pdf]) and was written as supplemental material for [this](https://www21.in.tum.de/teaching/sar/SS20/8.pdf) seminar paper.
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
The grammar for these strings is based on standard separation logic formulæ (definitions in order of priority):

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
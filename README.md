# alice_rs
Alice_rs is a small proof-of-concept reference implementation of a decision procedure for [A Decidable Fragment of Separation Logic](http://www0.cs.ucl.ac.uk/staff/p.ohearn/papers/unroll_collapse_withproofs.pdf]).
The name comes from a wrong pronounciation of the ls structure. The correct pronounciation would be "list structure" but it could also be read as "al-as structure" which sounds a bit like alice.

## Usage 
Alice_rs takes only an entailment as a string command line argument like this:
`alice "[here goes the entailment]"` <br />
Example: `alice "And[Neq(x,y)]|SepConj[x->y,y->Nil] |- True|SepConj[ls(x, Nil)]"` <br/>
The grammar for these strings is based on the standard separation logic formulæ (definitions in order of priority):

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


## Results
If the programm returns nothing, the entailment is valid.
Otherwise either a parser error occured or the entailment is found invalid.
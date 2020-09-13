use crate::datastructures::*;
use combine::{
    between,
    error::ParseError,
    many1,
    parser::{
        char::{char, letter, spaces, string},
        error::Silent,
        sequence::Skip,
        token::Token,
    },
    sep_by, tokens, Parser, Stream,
};

fn lex_char<Input>(c: char) -> Skip<Token<Input>, Silent<impl Parser<Input, Output = ()>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let skip_spaces = || spaces().silent();
    char(c).skip(skip_spaces())
}

fn parse_atomic_val<Input, T>(val_token: &'static str, val: T) -> impl Parser<Input, Output = T>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    T: Clone + 'static,
{
    tokens(
        |l, r| l.eq_ignore_ascii_case(&r),
        val_token,
        val_token.chars(),
    )
    .map(move |_| val.clone())
}

fn parse_pair<Input, L, R>(
    parse_left: impl Parser<Input, Output = L>,
    parse_right: impl Parser<Input, Output = R>,
) -> impl Parser<Input, Output = (L, R)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let left_right = (parse_left, lex_char(','), parse_right).map(|(l, _, r)| (l, r));
    let pair = between(lex_char('('), lex_char(')'), left_right);
    (pair).map(|pair_raw| pair_raw)
}

fn parse_vec<Input, T>(
    parse_t: impl Parser<Input, Output = T>,
) -> impl Parser<Input, Output = Vec<T>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let comma_list = sep_by::<Vec<T>, _, _, _>(parse_t, lex_char(','));
    let array = between(lex_char('['), lex_char(']'), comma_list);
    (array).map(|elements| elements)
}

fn parse_expr<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let parse_nil = parse_atomic_val("Nil", Expr::Nil);
    let parse_var = (many1::<Vec<_>, _, _>(letter()))
        .map(|name| Expr::new_var(&name.into_iter().collect::<String>()));
    parse_nil.or(parse_var)
}

fn parse_op<Input>() -> impl Parser<Input, Output = Op>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        string("Eq").or(string("Neq")),
        spaces(),
        parse_pair(parse_expr(), parse_expr()),
    )
        .map(|(s, _, (l, r))| {
            if s == "Eq" {
                Op::AtomEq(l, r)
            } else {
                Op::AtomNeq(l, r)
            }
        })
}

fn parse_pure<Input>() -> impl Parser<Input, Output = Pure>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let parse_true = parse_atomic_val("True", Pure::True);
    let parse_and = (string("And"), parse_vec(parse_op())).map(|(_, pure_vac)| Pure::And(pure_vac));
    parse_true.or(parse_and)
}

fn parse_atom_spatial<Input>() -> impl Parser<Input, Output = AtomSpatial>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let parse_points_to = (parse_expr(), spaces(), string("->"), spaces(), parse_expr())
        .map(|(l, _, _, _, r)| AtomSpatial::PointsTo(l, r));
    let parse_ls = (
        string("ls"),
        spaces(),
        parse_pair(parse_expr(), parse_expr()),
    )
        .map(|(_, _, (l, r))| AtomSpatial::LS(l, r));
    parse_ls.or(parse_points_to)
}

fn parse_spatial<Input>() -> impl Parser<Input, Output = Spatial>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let parse_sep_conj = (string("SepConj"), parse_vec(parse_atom_spatial()))
        .map(|(_, atom_sp_vec)| Spatial::SepConj(atom_sp_vec));
    let parse_emp = parse_atomic_val("Emp", Spatial::Emp);
    parse_sep_conj.or(parse_emp)
}

fn parse_formula<Input>() -> impl Parser<Input, Output = Formula>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (parse_pure(), lex_char('|'), parse_spatial()).map(|(pure, _, spatial)| Formula(pure, spatial))
}

/// Just a simple parser for entailments based on parser combinators

pub fn parse_entailment<Input>() -> impl Parser<Input, Output = Entailment>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        parse_formula(),
        spaces(),
        string("|-"),
        spaces(),
        parse_formula(),
    )
        .map(|(antecedent, _, _, _, consequent)| Entailment {
            antecedent,
            consequent,
        })
}

#[test]
fn tst() {
    let neq = parse_op().parse("Neq(x,y)");
    assert!(neq.is_ok());
    assert_eq!(
        neq.unwrap().0,
        Op::AtomNeq(Expr::new_var("x"), Expr::new_var("y"))
    );

    let and = parse_pure().parse("And[Neq(x,y)]");
    assert!(and.is_ok());
    assert_eq!(
        and.unwrap().0,
        Pure::And(vec![Op::AtomNeq(Expr::new_var("x"), Expr::new_var("y"))]),
    );

    let points_to1 = parse_atom_spatial().parse("x->y");
    assert!(points_to1.is_ok());
    assert_eq!(
        points_to1.unwrap().0,
        AtomSpatial::PointsTo(Expr::new_var("x"), Expr::new_var("y"))
    );

    let pointsto2 = parse_atom_spatial().parse("y->Nil");
    assert!(pointsto2.is_ok());
    assert_eq!(
        pointsto2.unwrap().0,
        AtomSpatial::PointsTo(Expr::new_var("y"), Expr::Nil)
    );

    let sepconj = parse_spatial().parse("SepConj[x->y,y->Nil]");
    assert!(sepconj.is_ok());
    assert_eq!(
        sepconj.unwrap().0,
        Spatial::SepConj(vec![
            AtomSpatial::PointsTo(Expr::new_var("x"), Expr::new_var("y")),
            AtomSpatial::PointsTo(Expr::new_var("y"), Expr::Nil),
        ])
    );

    let form = parse_formula().parse("And[Neq(x,y)]|SepConj[x->y,y->Nil]");
    assert!(form.is_ok());
    assert_eq!(
        form.unwrap().0,
        Formula(
            Pure::And(vec![Op::AtomNeq(Expr::new_var("x"), Expr::new_var("y"))]),
            Spatial::SepConj(vec![
                AtomSpatial::PointsTo(Expr::new_var("x"), Expr::new_var("y")),
                AtomSpatial::PointsTo(Expr::new_var("y"), Expr::Nil),
            ]),
        )
    );

    let p_true = parse_pure().parse("True");
    assert!(p_true.is_ok());
    assert_eq!(p_true.unwrap().0, Pure::True);

    let ls = parse_atom_spatial().parse("ls(x, Nil)");
    assert!(ls.is_ok());
    assert_eq!(
        ls.unwrap().0,
        AtomSpatial::LS(Expr::new_var("x"), Expr::Nil)
    );

    let expected = Entailment {
        antecedent: Formula(
            Pure::And(vec![Op::AtomNeq(Expr::new_var("x"), Expr::new_var("y"))]),
            Spatial::SepConj(vec![
                AtomSpatial::PointsTo(Expr::new_var("x"), Expr::new_var("y")),
                AtomSpatial::PointsTo(Expr::new_var("y"), Expr::Nil),
            ]),
        ),
        consequent: Formula(
            Pure::True,
            Spatial::SepConj(vec![AtomSpatial::LS(Expr::new_var("x"), Expr::Nil)]),
        ),
    };

    let parsed =
        parse_entailment().parse("And[Neq(x,y)]|SepConj[x->y,y->Nil] |- True|SepConj[ls(x, Nil)]");
    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap().0);
}

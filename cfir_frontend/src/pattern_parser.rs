use cfir::{
  rewriter::pattern::{Catch, OpPat, OpPatHand, ValuePat},
  symbol::{Name, Symbol},
  value::{Argument, Constant, Label, Order},
};
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../docs/pattern.pest"]
pub struct Pattern {}

pub type ParseError = pest::error::Error<Rule>;

pub trait PatternParseFrom
where
  Self: std::marker::Sized,
{
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self;
}

macro_rules! next {
  ($pairs:expr, $path:expr) => {
    PatternParseFrom::parse_from($pairs.next().unwrap(), $path)
  };
}

impl PatternParseFrom for Vec<Symbol> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::name_bind);
    pair
      .into_inner()
      .map(|pair| PatternParseFrom::parse_from(pair, path))
      .collect()
  }
}

pub fn op_def_pat_parse_from(pair: Pair<Rule>, path: &str) -> OpPatHand {
  debug_assert_eq!(pair.as_rule(), Rule::op_def_pat);
  let mut pairs = pair.into_inner();
  // let pair = pairs.next().unwrap();
  // if let Some(pair1) = pairs.next() {
  // let name = CFIRParseFrom::parse_from(pair, path);
  // let name = Some(name);
  // let op: OpHand = CFIRParseFrom::parse_from(pair1, path);
  // op.as_ref().borrow_mut().def = name.clone();
  // op
  // } else {
  // CFIRParseFrom::parse_from(pair, path)
  // }
  let defs: Vec<Symbol> = next!(pairs, path);
  let mut op: OpPat = next!(pairs, path);
  op.2 = defs;
  OpPatHand::new(op)
}

impl PatternParseFrom for OpPatHand {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    OpPatHand::new(PatternParseFrom::parse_from(pair, path))
  }
}

impl PatternParseFrom for OpPat {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::op_pat);
    let mut pairs = pair.into_inner();
    let opcode: Name = next!(pairs, path);
    let uses = next!(pairs, path);
    // let attr: HashMap<Symbol, Constant> = next!(pairs, path);
    OpPat(opcode, uses, vec![])
  }
}

impl PatternParseFrom for Vec<Catch<ValuePat>> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::uses);
    pair
      .into_inner()
      .map(|pair| PatternParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl PatternParseFrom for Catch<ValuePat> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::catch);
    let pair0 = pair.into_inner().next().unwrap();
    let mut pairs = pair0.clone().into_inner();
    let name = if pair0.as_rule() == Rule::catch_0 {
      Some(next!(pairs, path))
    } else {
      None
    };
    let value: Option<ValuePat> = pairs
      .next()
      .map(|pair| PatternParseFrom::parse_from(pair, path));
    Self(value, name)
  }
}

impl PatternParseFrom for Name {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::name);
    let mut pairs = pair.into_inner();
    let sym0 = next!(pairs, path);
    if let Some(pair) = pairs.next() {
      let sym1 = PatternParseFrom::parse_from(pair, path);
      Self(Some(sym0), sym1)
    } else {
      Self(None, sym0)
    }
  }
}

impl PatternParseFrom for ValuePat {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::value);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
      Rule::symbol_or_op_pat => {
        let pair = pair.into_inner().next().unwrap();
        if pair.as_rule() == Rule::op_pat {
          ValuePat::Use(PatternParseFrom::parse_from(pair, path), 0)
        } else {
          // if pair.as_rule() == Rule::op
          ValuePat::Input(PatternParseFrom::parse_from(pair, path))
        }
      },
      Rule::constant => ValuePat::Const(PatternParseFrom::parse_from(pair, path)),
      Rule::label => ValuePat::Label(PatternParseFrom::parse_from(pair, path)),
      Rule::argument => ValuePat::Argument(PatternParseFrom::parse_from(pair, path)),
      _ => unreachable!(),
    }
  }
}

impl PatternParseFrom for Label {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::label);
    let mut pairs = pair.into_inner();
    Label(next!(pairs, path))
  }
}

impl PatternParseFrom for Argument {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::argument);
    let mut pairs = pair.into_inner();
    let order = next!(pairs, path);
    let sym = next!(pairs, path);
    Argument(sym, order)
  }
}

impl PatternParseFrom for Option<Order> {
  fn parse_from(_pair: Pair<Rule>, _path: &str) -> Self {
    // fixme: maybe
    None
  }
}

impl PatternParseFrom for Constant {
  fn parse_from(pair: Pair<Rule>, _: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::constant);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
      Rule::int_lit => str::parse(pair.as_str()).map(Constant::Int).unwrap(),
      Rule::uint_lit => str::parse(pair.as_str()).map(Constant::Uint).unwrap(),
      Rule::bool_lit => str::parse(pair.as_str()).map(Constant::Bool).unwrap(),
      Rule::string_lit => str::parse(pair.as_str()).map(Constant::String).unwrap(),
      _ => unreachable!(),
    }
  }
}

impl PatternParseFrom for Symbol {
  fn parse_from(pair: Pair<Rule>, _: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::symbol);
    Symbol::new(pair.as_str())
  }
}

#[macro_export]
macro_rules! pat {
  ($src:expr) => {{
    use cfir::rewriter::pattern::OpPat;
    use cfir_frontend::pattern_parser::Pattern;
    use cfir_frontend::pattern_parser::{PatternParseFrom, Rule};
    use pest::Parser;
    let pair = Pattern::parse(Rule::op_pat, $src).unwrap();
    pair
      .into_iter()
      .map(|pair| -> OpPat { PatternParseFrom::parse_from(pair, "<test>") })
      .next()
      .unwrap()
  }};
}

#[macro_export]
macro_rules! value_pat {
  ($src:expr) => {{
    use cfir::rewriter::pattern::ValuePat;
    use cfir_frontend::pattern_parser::Pattern;
    use cfir_frontend::pattern_parser::{PatternParseFrom, Rule};
    use pest::Parser;
    let pair = Pattern::parse(Rule::value, $src).unwrap();
    pair
      .into_iter()
      .map(|pair| -> ValuePat { PatternParseFrom::parse_from(pair, "<test>") })
      .next()
      .unwrap()
  }};
}

mod test {
  #[test]
  fn test_parser() {
    use pest::Parser;

    use crate::pattern_parser::{Pattern, PatternParseFrom, Rule};
    use cfir::rewriter::pattern::OpPat;

    let src = "add(sub(_, ?b), ?b)";
    let pair = Pattern::parse(Rule::op_pat, src).unwrap();
    let r: Vec<OpPat> = pair
      .into_iter()
      .map(|pair| -> OpPat { PatternParseFrom::parse_from(pair, "<test>") })
      .collect();
    for i in r {
      println!("{:?}", i);
    }
  }
}

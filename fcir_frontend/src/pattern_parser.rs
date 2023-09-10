use fcir::{
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

pub trait FcirParseFrom
where
  Self: std::marker::Sized,
{
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self;
}

macro_rules! next {
  ($pairs:expr, $path:expr) => {
    FcirParseFrom::parse_from($pairs.next().unwrap(), $path)
  };
}

impl FcirParseFrom for OpPatHand {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    OpPatHand::new(FcirParseFrom::parse_from(pair, path))
  }
}

impl FcirParseFrom for OpPat {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::op_pat);
    let mut pairs = pair.into_inner();
    let opcode: Name = next!(pairs, path);
    let uses = next!(pairs, path);
    // let attr: HashMap<Symbol, Constant> = next!(pairs, path);
    OpPat(opcode, uses)
  }
}

impl FcirParseFrom for Vec<Catch<ValuePat>> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::uses);
    pair
      .into_inner()
      .map(|pair| FcirParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl FcirParseFrom for Catch<ValuePat> {
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
      .map(|pair| FcirParseFrom::parse_from(pair, path));
    Self(value, name)
  }
}

impl FcirParseFrom for Name {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::name);
    let mut pairs = pair.into_inner();
    let sym0 = next!(pairs, path);
    if let Some(pair) = pairs.next() {
      let sym1 = FcirParseFrom::parse_from(pair, path);
      Self(Some(sym0), sym1)
    } else {
      Self(None, sym0)
    }
  }
}

impl FcirParseFrom for ValuePat {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::value);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
      Rule::symbol_or_op_pat => {
        let pair = pair.into_inner().next().unwrap();
        if pair.as_rule() == Rule::op_pat {
          ValuePat::Use(FcirParseFrom::parse_from(pair, path))
        } else {
          // if pair.as_rule() == Rule::op
          ValuePat::Input(FcirParseFrom::parse_from(pair, path))
        }
      },
      Rule::constant => ValuePat::Const(FcirParseFrom::parse_from(pair, path)),
      Rule::label => ValuePat::Label(FcirParseFrom::parse_from(pair, path)),
      Rule::argument => ValuePat::Argument(FcirParseFrom::parse_from(pair, path)),
      _ => unreachable!(),
    }
  }
}

impl FcirParseFrom for Label {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::label);
    let mut pairs = pair.into_inner();
    Label(next!(pairs, path))
  }
}

impl FcirParseFrom for Argument {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::argument);
    let mut pairs = pair.into_inner();
    let order = next!(pairs, path);
    let sym = next!(pairs, path);
    Argument(sym, order)
  }
}

impl FcirParseFrom for Option<Order> {
  fn parse_from(_pair: Pair<Rule>, _path: &str) -> Self {
    // fixme: maybe
    None
  }
}

impl FcirParseFrom for Constant {
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

impl FcirParseFrom for Symbol {
  fn parse_from(pair: Pair<Rule>, _: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::symbol);
    Symbol::new(pair.as_str())
  }
}

use std::collections::HashMap;

use pest::iterators::Pair;
use pest_derive::Parser;

use fcir::{
  block::{Block, Region},
  op::{Op, OpHand},
  symbol::{Name, Symbol},
  types::{FuncType, GenericType, Type, TypeOrConst},
  value::{Argument, Constant, Label, Order, Value},
};

#[derive(Parser)]
#[grammar = "../docs/fcir.pest"]
pub struct Fcir {}

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

impl FcirParseFrom for (Option<Symbol>, OpHand) {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    let op = op_def_parse_from(pair, path);
    let name = op.as_ref().borrow().def.clone();
    (name, op)
  }
}

pub fn op_def_parse_from(pair: Pair<Rule>, path: &str) -> OpHand {
  debug_assert_eq!(pair.as_rule(), Rule::op_def);
  let mut pairs = pair.into_inner();
  let pair = pairs.next().unwrap();
  if let Some(pair1) = pairs.next() {
    let name = FcirParseFrom::parse_from(pair, path);
    let name = Some(name);
    let op: OpHand = FcirParseFrom::parse_from(pair1, path);
    op.as_ref().borrow_mut().def = name.clone();
    op
  } else {
    FcirParseFrom::parse_from(pair, path)
  }
}

impl FcirParseFrom for OpHand {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    OpHand::new(FcirParseFrom::parse_from(pair, path))
  }
}

impl FcirParseFrom for Op {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::op);
    let mut pairs = pair.into_inner();
    let opcode: Name = next!(pairs, path);
    let uses: Vec<Value> = next!(pairs, path);
    let attr: HashMap<Symbol, Constant> = next!(pairs, path);
    let region: Region = next!(pairs, path);
    let sign: FuncType = next!(pairs, path);
    Self {
      opcode,
      def: None,
      uses,
      attr,
      region,
      sign,
    }
  }
}

impl FcirParseFrom for Vec<Value> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::uses);
    pair
      .into_inner()
      .map(|pair| Value::parse_from(pair, path))
      .collect()
  }
}

impl FcirParseFrom for HashMap<Symbol, Constant> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::attr);
    pair
      .into_inner()
      .map(|pair| FcirParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl FcirParseFrom for (Symbol, Constant) {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::key_constant_pair);
    let mut pairs = pair.into_inner();
    let key = next!(pairs, path);
    let constant = next!(pairs, path);
    (key, constant)
  }
}

impl FcirParseFrom for Region {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::region);
    pair
      .into_inner()
      .map(|pair| FcirParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl FcirParseFrom for (Option<Symbol>, Block) {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::labeld_block);
    let mut pairs = pair.into_inner();
    let (sym, argu): (Option<Symbol>, HashMap<Symbol, Type>) =
      block_head_opt_parse_from(pairs.next().unwrap(), path);
    let mut block: Block = next!(pairs, path);
    block.0 = sym.clone();
    block.1 = argu;
    (sym, block)
  }
}

impl FcirParseFrom for Block {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::block);
    let opdefs = pair
      .into_inner()
      .map(|pair| op_def_parse_from(pair, path))
      .collect();
    Block(None, HashMap::new(), opdefs)
  }
}

impl FcirParseFrom for Vec<OpHand> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    pair
      .into_inner()
      .map(|pair| FcirParseFrom::parse_from(pair, path))
      .collect()
  }
}

fn block_head_opt_parse_from(
  pair: Pair<Rule>,
  path: &str,
) -> (Option<Symbol>, HashMap<Symbol, Type>) {
  debug_assert_eq!(pair.as_rule(), Rule::block_head_opt);
  if let Some(pair) = pair.into_inner().next() {
    FcirParseFrom::parse_from(pair, path)
  } else {
    (None, HashMap::new())
  }
}

impl FcirParseFrom for (Option<Symbol>, HashMap<Symbol, Type>) {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::block_head);
    if let Some(pair) = pair.into_inner().next() {
      let mut pairs = pair.into_inner();
      let pair0 = pairs.next().unwrap();
      if let Some(pair1) = pairs.next() {
        let label = FcirParseFrom::parse_from(pair0, path);
        let args: HashMap<Symbol, Type> = FcirParseFrom::parse_from(pair1, path);
        (Some(label), args)
      } else {
        let args: HashMap<Symbol, Type> = FcirParseFrom::parse_from(pair0, path);
        (None, args)
      }
    } else {
      (None, HashMap::new())
    }
  }
}

impl FcirParseFrom for HashMap<Symbol, Type> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::block_argument);
    pair
      .into_inner()
      .map(|pair| FcirParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl FcirParseFrom for (Symbol, Type) {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::symbol_type_pair);
    let mut pairs = pair.into_inner();
    let name = next!(pairs, path);
    let _type = next!(pairs, path);
    (name, _type)
  }
}

impl FcirParseFrom for Box<Type> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    Box::new(FcirParseFrom::parse_from(pair, path))
  }
}

impl FcirParseFrom for Type {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::_type);
    let pair = pair.into_inner().next().unwrap();
    if pair.as_rule() == Rule::generic_type {
      Type::GenericType(FcirParseFrom::parse_from(pair, path))
    } else {
      // pair.as_rule() == Rule::func_type
      Type::FuncType(FcirParseFrom::parse_from(pair, path))
    }
  }
}

impl FcirParseFrom for GenericType {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::generic_type);
    let mut pairs = pair.into_inner();
    let name = next!(pairs, path);
    if let Some(pair) = pairs.next() {
      GenericType {
        name,
        args: FcirParseFrom::parse_from(pair, path),
      }
    } else {
      GenericType { name, args: vec![] }
    }
  }
}

impl FcirParseFrom for Vec<TypeOrConst> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::type_argument);
    pair
      .into_inner()
      .map(|pair| FcirParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl FcirParseFrom for TypeOrConst {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::type_or_const);
    if pair.as_rule() == Rule::_type {
      TypeOrConst::Type(FcirParseFrom::parse_from(pair, path))
    } else {
      // pair.as_rule() == Rule::constant
      TypeOrConst::Const(FcirParseFrom::parse_from(pair, path))
    }
  }
}

impl FcirParseFrom for FuncType {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::func_type);
    let mut pairs = pair.into_inner();
    let input = next!(pairs, path);
    let output = next!(pairs, path);
    FuncType(input, output)
  }
}

impl FcirParseFrom for Vec<Type> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::type_list);
    pair
      .into_inner()
      .map(|pair| FcirParseFrom::parse_from(pair, path))
      .collect()
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

impl FcirParseFrom for Value {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::value);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
      Rule::symbol_or_op => {
        let pair = pair.into_inner().next().unwrap();
        if pair.as_rule() == Rule::symbol {
          Value::Input(FcirParseFrom::parse_from(pair, path))
        } else {
          // if pair.as_rule() == Rule::op
          Value::Use(FcirParseFrom::parse_from(pair, path))
        }
      },
      Rule::constant => Value::Const(FcirParseFrom::parse_from(pair, path)),
      Rule::label => Value::Label(FcirParseFrom::parse_from(pair, path)),
      Rule::argument => Value::Argument(FcirParseFrom::parse_from(pair, path)),
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

mod test {
  #[test]
  fn test_parser() {
    use pest::Parser;

    use super::Fcir;
    use crate::fcir_parser::{FcirParseFrom, Rule};
    use fcir::block::Block;

    let src = "fn.def (a) [ inline: true ] {
      r = arthi.add (a, 1): (int, int) -> int
      fn.ret (r): (int) -> never
  }: () -> (int, int) -> int";
    let pair = Fcir::parse(Rule::block, src).unwrap();
    let r: Vec<Block> = pair
      .into_iter()
      .map(|pair| -> Block { FcirParseFrom::parse_from(pair, "<test>") })
      .collect();
    for i in r {
      println!("{:?}", i);
    }
  }
}

use std::collections::HashMap;

use pest::iterators::Pair;
use pest_derive::Parser;

use cfir::{
  block::{Block, Region},
  op::{Op, OpHand},
  symbol::{Name, Symbol},
  types::{FuncType, GenericType, Type, TypeOrConst},
  value::{Argument, Constant, Label, Order, Value},
};

#[derive(Parser)]
#[grammar = "../docs/cfir.pest"]
pub struct CFIR {}

pub type ParseError = pest::error::Error<Rule>;

pub trait CFIRParseFrom
where
  Self: std::marker::Sized,
{
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self;
}

macro_rules! next {
  ($pairs:expr, $path:expr) => {
    CFIRParseFrom::parse_from($pairs.next().unwrap(), $path)
  };
}

/*
impl CFIRParseFrom for (Option<Symbol>, OpHand) {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    let op = op_def_parse_from(pair, path);
    let name = op.as_ref().borrow().def.clone();
    (name, op)
  }
}
 */

impl CFIRParseFrom for Vec<Symbol> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::name_bind);
    pair
      .into_inner()
      .map(|pair| CFIRParseFrom::parse_from(pair, path))
      .collect()
  }
}

pub fn op_def_parse_from(pair: Pair<Rule>, path: &str) -> OpHand {
  debug_assert_eq!(pair.as_rule(), Rule::op_def);
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
  let mut op: Op = next!(pairs, path);
  op.defs = defs;
  OpHand::new(op)
}

impl CFIRParseFrom for OpHand {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    OpHand::new(CFIRParseFrom::parse_from(pair, path))
  }
}

impl CFIRParseFrom for Op {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::op);
    let mut pairs = pair.into_inner();
    let opcode: Name = next!(pairs, path);
    let uses: Vec<Value> = next!(pairs, path);
    let attr: HashMap<Symbol, Constant> = next!(pairs, path);
    let region: Region = next!(pairs, path);
    // let sign: FuncType = next!(pairs, path);
    let sign: Vec<Type> = next!(pairs, path);
    Self {
      opcode,
      // def: None,
      defs: vec![],
      uses,
      attr,
      region,
      sign,
    }
  }
}

impl CFIRParseFrom for Vec<Value> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::uses);
    pair
      .into_inner()
      .map(|pair| Value::parse_from(pair, path))
      .collect()
  }
}

impl CFIRParseFrom for HashMap<Symbol, Constant> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::attr);
    pair
      .into_inner()
      .map(|pair| CFIRParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl CFIRParseFrom for (Symbol, Constant) {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::key_constant_pair);
    let mut pairs = pair.into_inner();
    let key = next!(pairs, path);
    let constant = next!(pairs, path);
    (key, constant)
  }
}

impl CFIRParseFrom for Region {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::region);
    pair
      .into_inner()
      .map(|pair| CFIRParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl CFIRParseFrom for (Option<Symbol>, Block) {
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

impl CFIRParseFrom for Block {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::block);
    let opdefs = pair
      .into_inner()
      .map(|pair| op_def_parse_from(pair, path))
      .collect();
    Block(None, HashMap::new(), opdefs)
  }
}

impl CFIRParseFrom for Vec<OpHand> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    pair
      .into_inner()
      .map(|pair| CFIRParseFrom::parse_from(pair, path))
      .collect()
  }
}

fn block_head_opt_parse_from(
  pair: Pair<Rule>,
  path: &str,
) -> (Option<Symbol>, HashMap<Symbol, Type>) {
  debug_assert_eq!(pair.as_rule(), Rule::block_head_opt);
  if let Some(pair) = pair.into_inner().next() {
    CFIRParseFrom::parse_from(pair, path)
  } else {
    (None, HashMap::new())
  }
}

impl CFIRParseFrom for (Option<Symbol>, HashMap<Symbol, Type>) {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::block_head);
    if let Some(pair) = pair.into_inner().next() {
      let mut pairs = pair.into_inner();
      let pair0 = pairs.next().unwrap();
      if let Some(pair1) = pairs.next() {
        let label = CFIRParseFrom::parse_from(pair0, path);
        let args: HashMap<Symbol, Type> = CFIRParseFrom::parse_from(pair1, path);
        (Some(label), args)
      } else {
        let args: HashMap<Symbol, Type> = CFIRParseFrom::parse_from(pair0, path);
        (None, args)
      }
    } else {
      (None, HashMap::new())
    }
  }
}

impl CFIRParseFrom for HashMap<Symbol, Type> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::block_argument);
    pair
      .into_inner()
      .map(|pair| CFIRParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl CFIRParseFrom for (Symbol, Type) {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::symbol_type_pair);
    let mut pairs = pair.into_inner();
    let name = next!(pairs, path);
    let _type = next!(pairs, path);
    (name, _type)
  }
}

impl CFIRParseFrom for Box<Type> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    Box::new(CFIRParseFrom::parse_from(pair, path))
  }
}

impl CFIRParseFrom for Type {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::_type);
    let pair = pair.into_inner().next().unwrap();
    if pair.as_rule() == Rule::generic_type {
      Type::GenericType(CFIRParseFrom::parse_from(pair, path))
    } else {
      // pair.as_rule() == Rule::func_type
      Type::FuncType(CFIRParseFrom::parse_from(pair, path))
    }
  }
}

impl CFIRParseFrom for GenericType {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::generic_type);
    let mut pairs = pair.into_inner();
    let name = next!(pairs, path);
    if let Some(pair) = pairs.next() {
      GenericType {
        name,
        args: CFIRParseFrom::parse_from(pair, path),
      }
    } else {
      GenericType { name, args: vec![] }
    }
  }
}

impl CFIRParseFrom for Vec<TypeOrConst> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::type_argument);
    pair
      .into_inner()
      .map(|pair| CFIRParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl CFIRParseFrom for TypeOrConst {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::type_or_const);
    if pair.as_rule() == Rule::_type {
      TypeOrConst::Type(CFIRParseFrom::parse_from(pair, path))
    } else {
      // pair.as_rule() == Rule::constant
      TypeOrConst::Const(CFIRParseFrom::parse_from(pair, path))
    }
  }
}

impl CFIRParseFrom for FuncType {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::func_type);
    let mut pairs = pair.into_inner();
    let input: Vec<Type> = next!(pairs, path);
    let output: Vec<Type> = next!(pairs, path);
    FuncType(input, output)
  }
}

impl CFIRParseFrom for Vec<Type> {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::type_list);
    pair
      .into_inner()
      .map(|pair| CFIRParseFrom::parse_from(pair, path))
      .collect()
  }
}

impl CFIRParseFrom for Name {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::name);
    let mut pairs = pair.into_inner();
    let sym0: Symbol = next!(pairs, path);
    if let Some(pair) = pairs.next() {
      let sym1: Symbol = CFIRParseFrom::parse_from(pair, path);
      Self(Some(sym0), sym1)
    } else {
      Self(None, sym0)
    }
  }
}

impl CFIRParseFrom for Value {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::value);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
      Rule::symbol_or_op => {
        let pair = pair.into_inner().next().unwrap();
        if pair.as_rule() == Rule::symbol {
          Value::Input(CFIRParseFrom::parse_from(pair, path))
        } else {
          // if pair.as_rule() == Rule::op
          // Value::Use(CFIRParseFrom::parse_from(pair, path))
          Value::Use(CFIRParseFrom::parse_from(pair, path), 0)
        }
      },
      Rule::constant => Value::Const(CFIRParseFrom::parse_from(pair, path)),
      Rule::label => Value::Label(CFIRParseFrom::parse_from(pair, path)),
      Rule::argument => Value::Argument(CFIRParseFrom::parse_from(pair, path)),
      _ => unreachable!(),
    }
  }
}

impl CFIRParseFrom for Label {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::label);
    let mut pairs = pair.into_inner();
    Label(next!(pairs, path))
  }
}

impl CFIRParseFrom for Argument {
  fn parse_from(pair: Pair<Rule>, path: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::argument);
    let mut pairs = pair.into_inner();
    let order: Option<Order> = next!(pairs, path);
    let sym: Symbol = next!(pairs, path);
    Argument(sym, order)
  }
}

impl CFIRParseFrom for Option<Order> {
  fn parse_from(_pair: Pair<Rule>, _path: &str) -> Self {
    // fixme: maybe
    None
  }
}

impl CFIRParseFrom for Constant {
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

impl CFIRParseFrom for Symbol {
  fn parse_from(pair: Pair<Rule>, _: &str) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::symbol);
    Symbol::new(pair.as_str())
  }
}

#[macro_export]
macro_rules! cfir_block {
  ($src:expr) => {{
    use cfir::block::Block;
    use cfir_frontend::cfir_parser::{CFIRParseFrom, Rule, CFIR};
    use pest::Parser;
    let pair = CFIR::parse(Rule::block, $src).unwrap();
    pair
      .into_iter()
      .map(|pair| -> Block { CFIRParseFrom::parse_from(pair, "<builtin>") })
      .next()
      .unwrap()
  }};
}

#[macro_export]
macro_rules! cfir_expr {
  ($src:expr) => {{
    use cfir::op::Op;
    use cfir_frontend::cfir_parser::{CFIRParseFrom, Rule, CFIR};
    use pest::Parser;
    let pair = CFIR::parse(Rule::op, $src).unwrap();
    pair
      .into_iter()
      .map(|pair| -> Op { CFIRParseFrom::parse_from(pair, "<builtin>") })
      .next()
      .unwrap()
  }};
}

#[macro_export]
macro_rules! value {
  ($src:expr) => {{
    use cfir::value::Value;
    use cfir_frontend::cfir_parser::{CFIRParseFrom, Rule, CFIR};

    use pest::Parser;
    let pair = cfir::parse(Rule::value, $src).unwrap();
    pair
      .into_iter()
      .map(|pair| -> Value { CFIRParseFrom::parse_from(pair, "<builtin>") })
      .next()
      .unwrap()
  }};
}

mod test {
  #[test]
  fn test_parser() {
    use crate::cfir_parser::{CFIRParseFrom, Rule, CFIR};
    use cfir::block::Block;
    use pest::Parser;

    let src = "fn.def (a) [ inline: true ] {
      r = arthi.add (a, 1): (int, int) -> int
      fn.ret (r): (int) -> never
  }: () -> (int) -> int";
    let pair = CFIR::parse(Rule::block, src).unwrap();
    let r: Vec<Block> = pair
      .into_iter()
      .map(|pair| -> Block { CFIRParseFrom::parse_from(pair, "<test>") })
      .collect();
    for i in r {
      println!("{:?}", i);
    }
  }
}

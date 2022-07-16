use self::bytecode::Bytecode;
use super::{
  handles::{GlobalSymbol, LocalSymbol, Symbol},
  types::{Type, TypeBindAttr},
};

pub mod bytecode;
pub mod three_ac;
pub mod two_ac;

#[derive(Debug, Clone)]
pub struct FunctionHeader {
  pub name:      GlobalSymbol,
  pub arguments: Vec<(LocalSymbol, TypeBindAttr)>,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
  pub header:         FunctionHeader,
  pub local_variable: Vec<(LocalSymbol, TypeBindAttr)>,
  pub local_structs:  Vec<(LocalSymbol, Type)>,
  // pub bodys: Vec<Bytecode>,
  pub basicblocks:    Vec<BasicBlock>,
}

/*
impl FunctionInfo {
    pub fn gen_layout(&self) -> FunctionDataLayout {

    }
}
 */

#[derive(Debug, Clone)]
pub struct FunctionDataLayout {}

#[derive(Debug, Clone)]
pub struct BasicBlock {
  pub label: Option<Symbol>,
  pub bodys: Vec<Bytecode>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BranchOp {
  IfNil,
  IfNonNil,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ICmpOp {
  Eq,
  Ne,
  Sge,
  Sgt,
  Sle,
  Slt,
  Uge,
  Ugt,
  Ule,
  Ult,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FCmpOp {
  False,
  Oeq,
  Oge,
  Ogt,
  Ole,
  Olt,
  One,
  Ord,
  True,
  Ueq,
  Uge,
  Ugt,
  Ule,
  Ult,
  Une,
  Uno,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Index {
  Index(usize),
  Symbol(Symbol),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndexList(pub Vec<Index>);

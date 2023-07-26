use crate::{
  symbol::{Name, Symbol},
  types::Type,
  value::Constant,
};

pub struct OpPat {
  pub opcode: Option<Name>,
  pub defs: Vec<CatchExpr>,
  pub uses: Vec<ValuePat>,
  pub ragion: Option<Catch>,
  pub sign: Option<Type>,
}

pub enum ValuePat {
  Const(Constant),
  Catch(CatchExpr),
}

pub struct CatchExpr(Option<SymbolPat>, Option<ValueType>);

pub enum ValueType {
  Use,
  Argument,
  Label,
}

pub enum SymbolPat {
  Symbol(Symbol),
  Catch(Catch),
}

pub type Catch = Symbol;

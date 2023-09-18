use crate::{
  symbol::{Name, Symbol},
  value::Constant,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Type {
  GenericType(GenericType),
  FuncType(FuncType),
  // Tuple(TupleType),
  // Union(UnionType),
}

impl Type {
  pub fn uninfered() -> Type {
    Type::GenericType(GenericType {
      name: Name(None, Symbol::new("uninfered")),
      args: vec![],
    })
  }

  pub fn any_type() -> Type {
    Type::GenericType(GenericType {
      name: Name(None, Symbol::new("any")),
      args: vec![],
    })
  }

  pub fn never_type() -> Type {
    Type::GenericType(GenericType {
      name: Name(None, Symbol::new("never")),
      args: vec![],
    })
  }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GenericType {
  pub name: Name,
  pub args: Vec<TypeOrConst>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeOrConst {
  Type(Type),
  Const(Constant),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FuncType(pub Vec<Type>, pub Box<Type>);

/*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleType (pub Vec<Type>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionType (pub Vec<Type>);
// */
// pub struct TypesPattern {}

use crate::{symbol::Name, value::Constant};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Type {
  TypeFunc(TypeFunc),
  FuncType(FuncType),
  // Tuple(TupleType),
  // Union(UnionType),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TypeFunc {
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

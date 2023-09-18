use cfvm_common::unbalanced_product;
use cfir::{
  op::{Op, OpHand},
  value::Value,
};

use crate::{
  eclass::{EClass, Id},
  enode::{ENode, EOp, EOpHand, RawENode},
};

pub trait Gencfir {
  type Output;
  fn gen_cfir(&self) -> Self::Output;
}

impl<D> Gencfir for EOpHand<D> {
  type Output = Vec<OpHand>;

  fn gen_cfir(&self) -> Self::Output {
    self.as_ref().borrow().gen_cfir()
  }
}

impl<D> Gencfir for EOp<D> {
  type Output = Vec<OpHand>;

  fn gen_cfir(&self) -> Self::Output {
    let uses = self.uses.iter().map(Gencfir::gen_cfir).collect::<Vec<_>>();
    let uses = uses.iter().fold(vec![], |a, b| unbalanced_product(&a, b));
    uses
      .into_iter()
      .map(|uses| {
        OpHand::new(Op {
          opcode: self.opcode.clone(),
          def: self.def.clone(),
          uses,
          attr: self.attr.clone(),
          region: self.region.clone(),
          sign: self.sign.clone(),
        })
      })
      .collect()
  }
}

impl<D> Gencfir for Id<D> {
  type Output = Vec<Value>;

  fn gen_cfir(&self) -> Self::Output {
    self.as_ref().borrow().gen_cfir()
  }
}

impl<D> Gencfir for EClass<D> {
  type Output = Vec<Value>;

  fn gen_cfir(&self) -> Self::Output {
    self
      .nodes
      .iter()
      .flat_map(|node| node.gen_cfir())
      .collect::<Vec<_>>()
  }
}

impl<D> Gencfir for ENode<D> {
  type Output = Vec<Value>;

  fn gen_cfir(&self) -> Self::Output {
    self.body.gen_cfir()
  }
}

impl<D> Gencfir for RawENode<D> {
  type Output = Vec<Value>;

  fn gen_cfir(&self) -> Self::Output {
    match self {
      RawENode::Use(op) => op.gen_cfir().into_iter().map(Value::Use).collect(),
      RawENode::Const(c) => vec![Value::Const(c.clone())],
      RawENode::Argument(a) => vec![Value::Argument(a.clone())],
      RawENode::Label(l) => vec![Value::Label(l.clone())],
      RawENode::Input(i) => vec![Value::Input(i.clone())],
    }
  }
}

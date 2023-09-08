use cfvm_common::unbalanced_product;
use fcir::{
  op::{Op, OpHand},
  value::Value,
};

use crate::{
  eclass::{EClass, Id},
  enode::{ENode, EOp, EOpHand, RawENode},
};

pub trait GenFcir {
  type Output;
  fn gen_fcir(&self) -> Self::Output;
}

impl<D> GenFcir for EOpHand<D> {
  type Output = Vec<OpHand>;

  fn gen_fcir(&self) -> Self::Output {
    self.as_ref().borrow().gen_fcir()
  }
}

impl<D> GenFcir for EOp<D> {
  type Output = Vec<OpHand>;

  fn gen_fcir(&self) -> Self::Output {
    let uses = self.uses.iter().map(GenFcir::gen_fcir).collect::<Vec<_>>();
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

impl<D> GenFcir for Id<D> {
  type Output = Vec<Value>;

  fn gen_fcir(&self) -> Self::Output {
    self.as_ref().borrow().gen_fcir()
  }
}

impl<D> GenFcir for EClass<D> {
  type Output = Vec<Value>;

  fn gen_fcir(&self) -> Self::Output {
    self
      .nodes
      .iter()
      .flat_map(|node| node.gen_fcir())
      .collect::<Vec<_>>()
  }
}

impl<D> GenFcir for ENode<D> {
  type Output = Vec<Value>;

  fn gen_fcir(&self) -> Self::Output {
    self.body.gen_fcir()
  }
}

impl<D> GenFcir for RawENode<D> {
  type Output = Vec<Value>;

  fn gen_fcir(&self) -> Self::Output {
    match self {
      RawENode::Use(op) => op.gen_fcir().into_iter().map(Value::Use).collect(),
      RawENode::Const(c) => vec![Value::Const(c.clone())],
      RawENode::Argument(a) => vec![Value::Argument(a.clone())],
      RawENode::Label(l) => vec![Value::Label(l.clone())],
      RawENode::Input(i) => vec![Value::Input(i.clone())],
    }
  }
}

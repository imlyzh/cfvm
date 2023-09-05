use fcir::op::Op;

use crate::{eclass::{Id, EClass}, enode::{ENode, RawENode, EOpHand, EOp}};



pub trait GenFcir {
  type Output;
  fn gen_ir(&self) -> Vec<Self::Output>;
}

impl<D> GenFcir for EOpHand<D> {
  type Output = Op;

  fn gen_ir(&self) -> Vec<Self::Output> {
    self.as_ref().borrow().gen_ir()
  }
}

impl<D> GenFcir for EOp<D> {
  type Output = Op;

  fn gen_ir(&self) -> Vec<Self::Output> {
    todo!()
  }
}

impl<D> GenFcir for Id<D> {
    type Output = Op;

    fn gen_ir(&self) -> Vec<Self::Output> {
      self.as_ref().borrow().gen_ir()
    }
}

impl<D> GenFcir for EClass<D> {
  type Output = Op;

  fn gen_ir(&self) -> Vec<Self::Output> {
    self.nodes.iter().flat_map(|node| node.gen_ir()).collect::<Vec<_>>()
  }
}

impl<D> GenFcir for ENode<D> {
  type Output = Op;

  fn gen_ir(&self) -> Vec<Self::Output> {
    self.body.gen_ir()
  }
}

impl<D> GenFcir for RawENode<D> {
  type Output = Op;

  fn gen_ir(&self) -> Vec<Self::Output> {
    match self {
      RawENode::Use(op) => op.gen_ir(),
        RawENode::Const(_) => vec![todo!()],
        RawENode::Argument(_) => vec![todo!()],
        RawENode::Label(_) => vec![todo!()],
    }
  }
}

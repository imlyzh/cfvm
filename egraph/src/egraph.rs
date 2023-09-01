use std::collections::HashMap;

use fcir::{
  form::Form,
  op::{Op, OpHand},
  value::Value,
};

use crate::{
  eclass::{EClass, Id},
  elike::ELike,
  enode::ENode,
};

#[derive(Debug, Default)]
pub struct EGraph<D> {
  root: Vec<Id<D>>,
  likes: ELike<D>,

  eclasses: Vec<EClass<D>>,
}

impl<D> EGraph<D> {
  pub fn new() -> Self {
    EGraph {
      root: Default::default(),
      eclasses: Default::default(),
      likes: Default::default(),
    }
  }

  pub fn add_op(&mut self, o: &Op) -> (Form, Id<D>) {
    todo!()
  }

  pub fn add_node(&mut self, node: Value) -> (Form, Id<D>) {
    let (f, r) = self.make_enode(node);

    todo!()
  }

  pub fn make_enode(&mut self, node: Value) -> (Form, ENode<D>) {
    match node {
      Value::Use(op) => {
        let (f, r) = self.add_op(op.as_ref());
        (f, ENode::Use(r))
      },
      Value::Const(n) => (Form::Atom, n.into()),
      Value::Argument(n) => (Form::Atom, n.into()),
      Value::Label(n) => (Form::Atom, n.into()),
    }
  }
}

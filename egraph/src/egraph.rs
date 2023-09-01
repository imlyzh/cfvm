use fcir::{form::Form, op::Op, value::Value};

use crate::{
  eclass::{EClass, Id},
  elike::ELike,
  enode::{ENode, EOp, EOpHand},
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
}

impl<D: Default> EGraph<D> {
  pub fn add_op(&mut self, o: &Op) -> (Form, Id<D>, EOpHand<D>) {
    let r = o.uses.iter().map(|i| self.add_node(i)).collect::<Vec<_>>();

    let form = r.iter().map(|(f, _)| f).cloned().collect::<Vec<Form>>();

    let uses = r.iter().map(|(_, i)| i).cloned().collect();

    let form = Form::Form(o.opcode.clone(), form);

    let eop = EOp {
      opcode: o.opcode.clone(),
      uses,
      attr: o.attr.clone(),
      ragion: o.ragion.clone(),
      sign: o.sign.clone(),
    };
    let eop = EOpHand::new(eop);
    let node = ENode::Use(eop.clone());

    let id = self.likes.add_node(&form, node);

    (form, id, eop)
  }

  pub fn add_node(&mut self, value: &Value) -> (Form, Id<D>) {
    let (f, node) = self.make_enode(value);
    let r = self.likes.add_node(&f, node);
    (f, r)
  }

  pub fn make_enode(&mut self, value: &Value) -> (Form, ENode<D>) {
    match value {
      Value::Use(op) => {
        let (form, _id, eop) = self.add_op(op.as_ref());
        (form, ENode::Use(eop))
      },
      Value::Const(n) => (Form::Atom, n.into()),
      Value::Argument(n) => (Form::Atom, n.into()),
      Value::Label(n) => (Form::Atom, n.into()),
    }
  }
}

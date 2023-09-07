use fcir::{op::Op, value::Value, rewriter::form::{Form, GetForm}};

use crate::{
  eclass::Id,
  elike::ELike,
  enode::{ENode, EOp, EOpHand, RawENode},
};

#[derive(Debug, Default)]
pub struct EGraph<D> {
  pub root: Vec<Id<D>>,
  pub likes: ELike<D>,
  pub eclasses: Vec<Id<D>>,
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
  pub fn add_op(&mut self, o: &Op) -> (Id<D>, EOpHand<D>) {
    let r = o.uses.iter().map(|i| self.add_value(i)).collect::<Vec<_>>();

    let form = r.iter().map(|(f, _)| f).cloned().collect::<Vec<Form>>();

    let uses = r.iter().map(|(_, i)| i).cloned().collect();

    let form = Form::Form(o.opcode.clone(), form);

    let eop = EOp {
      // form_cache: RefCell::new(Some(form)),
      form_cache: form,
      opcode: o.opcode.clone(),
      uses,
      attr: o.attr.clone(),
      region: o.region.clone(),
      sign: o.sign.clone(),
    };
    let eop = EOpHand::new(eop);
    let node = RawENode::Use(eop.clone());

    let id = self.add_raw_node(node);

    (id, eop)
  }

  pub fn add_value(&mut self, value: &Value) -> (Form, Id<D>) {
    let node = self.make_enode(value);
    let f = node.get_form();
    let r = self.add_raw_node(node);
    (f, r)
  }

  pub fn add_node(&mut self, node: ENode<D>) -> Id<D> {
    let input_id = node.get_id();
    let id = self.likes.add_node(&node.get_form(), node);
    if id != input_id {
      self.eclasses.push(id.clone());
    }
    id
  }

  pub fn add_raw_node(&mut self, node: RawENode<D>) -> Id<D> {
    let id = self.likes.add_raw_node(&node.get_form(), node);
    self.eclasses.push(id.clone());
    id
  }

  pub fn make_enode(&mut self, value: &Value) -> RawENode<D> {
    match value {
      Value::Use(op) => {
        let (_id, eop) = self.add_op(op.as_ref());
        RawENode::Use(eop)
      },
      Value::Const(n) => n.into(),
      Value::Argument(n) => n.into(),
      Value::Label(n) => n.into(),
    }
  }
}

use std::collections::HashMap;

use cfir::{
  block::Region,
  op::Attr,
  rewriter::{
    form::{Form, GetForm},
    pattern::{Catch, OpPat, OpPatHand, ValuePat},
  },
  symbol::Symbol,
  types::{FuncType, Type},
};

use crate::{
  egraph::EGraph,
  enode::{ENode, EOp, EOpHand, RawENode},
};

type MatchRecord<D> = HashMap<Symbol, ENode<D>>;

pub trait Rewriter<D> {
  type Output;
  fn rewrite(&self, record: &MatchRecord<D>, egraph: &mut EGraph<D>) -> Self::Output;
}

impl<D: Default> Rewriter<D> for OpPat {
  type Output = Option<EOp<D>>;

  fn rewrite(&self, record: &MatchRecord<D>, egraph: &mut EGraph<D>) -> Self::Output {
    let uses = self
      .1
      .iter()
      .map(|catch| catch.rewrite(record, egraph))
      .collect::<Option<Vec<_>>>()?;

    let forms = uses.iter().map(GetForm::get_form).collect();

    let uses = uses.iter().map(|node| node.get_id()).collect();

    Some(EOp {
      form_cache: Form::Form(self.0.clone(), forms),
      opcode: self.0.clone(),
      // def: None, // FIXME: gen new id
      defs: vec![], // FIXME: gen new id
      uses,
      attr: Attr::new(),
      region: Region::new(),
      // sign: self.2.clone(),
      // sign: FuncType(vec![], vec![Type::any_type()]), // FIXME: type inference
      sign: vec![Type::any_type()],
    })
  }
}

impl<D: Default> Rewriter<D> for OpPatHand {
  type Output = Option<EOpHand<D>>;

  fn rewrite(&self, record: &MatchRecord<D>, egraph: &mut EGraph<D>) -> Self::Output {
    self
      .as_ref()
      .borrow()
      .rewrite(record, egraph)
      .map(EOpHand::new)
  }
}

impl<D: Default> Rewriter<D> for Catch<ValuePat> {
  type Output = Option<ENode<D>>;

  fn rewrite(&self, record: &MatchRecord<D>, egraph: &mut EGraph<D>) -> Self::Output {
    match (&self.0, &self.1) {
      (None, None) => None,
      (None, Some(sym)) => record.get(sym).cloned(),
      (Some(pat), None) => pat.rewrite(record, egraph),
      (Some(_pat), Some(_sym)) => {
        todo!()
      },
    }
  }
}

impl<D: Default> Rewriter<D> for ValuePat {
  type Output = Option<ENode<D>>;

  fn rewrite(&self, record: &MatchRecord<D>, egraph: &mut EGraph<D>) -> Self::Output {
    let node = match self {
      ValuePat::Use(u, offset) => RawENode::Use(u.rewrite(record, egraph)?, *offset),
      ValuePat::Const(v) => RawENode::Const(v.clone()),
      ValuePat::Argument(v) => RawENode::Argument(v.clone()),
      ValuePat::Label(v) => RawENode::Label(v.clone()),
      ValuePat::Input(v) => RawENode::Input(v.clone()),
    };
    let (_id, r) = egraph.add_raw_node(node);
    Some(r)
  }
}

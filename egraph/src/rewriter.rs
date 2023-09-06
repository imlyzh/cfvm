use std::collections::HashMap;

use cfvm_common::unbalanced_product;
use fcir::{
  block::Region,
  op::Attr,
  symbol::{Name, Symbol},
  types::FuncType,
  value::Value,
};

use crate::{
  eclass::Id,
  egraph::EGraph,
  enode::{EOp, EOpHand, RawENode},
  form::Form,
  pattern::MatchValue,
};

pub trait Rewriter<D> {
  type Output;
  fn rewrite(&self, res: &HashMap<Symbol, MatchValue<D>>, egraph: &mut EGraph<D>) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpTemplate(pub Name, pub Vec<Insert<Value>>, pub FuncType);

impl<D: Default> Rewriter<D> for OpTemplate {
  type Output = Vec<Id<D>>;
  fn rewrite(&self, res: &HashMap<Symbol, MatchValue<D>>, egraph: &mut EGraph<D>) -> Self::Output {
    let uses = self
      .1
      .iter()
      .map(|i| i.rewrite(res, egraph))
      .collect::<Vec<_>>();
    let uses = uses
      .into_iter()
      .fold(vec![], |a, b| unbalanced_product(&a, &b));

    let mut r = vec![];
    for uses in uses.into_iter() {
      let forms = uses
        .iter()
        .map(|id| id.get_forms())
        .fold(vec![], |a, b| unbalanced_product(&a, &b));
      let append = forms.into_iter().map(|forms| {
        let eop = EOp {
          // form_cache: RefCell::new(Some(form)),
          form_cache: Form::Form(self.0.clone(), forms),
          opcode: self.0.clone(),
          uses: uses.clone(),
          attr: Attr::new(),
          region: Region::new(),
          sign: self.2.clone(),
        };
        let eop = EOpHand::new(eop);
        let node = RawENode::Use(eop.clone());
        egraph.add_raw_node(node)
      });
      r.extend(append)
    }
    r
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Insert<T> {
  Var(Symbol),
  Use(OpTemplate),
  Lit(T),
}

impl<D: Default> Rewriter<D> for Insert<Value> {
  type Output = Vec<Id<D>>;
  fn rewrite(&self, res: &HashMap<Symbol, MatchValue<D>>, egraph: &mut EGraph<D>) -> Self::Output {
    match self {
      Insert::Use(op) => op.rewrite(res, egraph),
      Insert::Var(sym) => res
        .get(sym)
        .into_iter()
        .map(|node| egraph.add_node(node.clone()))
        .collect(),
      Insert::Lit(value) => vec![egraph.add_value(value).1],
    }
  }
}

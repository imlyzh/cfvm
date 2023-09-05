use std::collections::HashMap;

use fcir::{symbol::{Name, Symbol}, value::Value};

use crate::{pattern::MatchValue, eclass::Id, egraph::EGraph, form::GetForm, enode::ENode};

pub trait Rewriter<D> {
  type Output;
  fn rewrite(&self, res: &HashMap<Symbol, MatchValue<D>>, egraph: &mut EGraph<D>) -> Option<Self::Output>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpTemplate(pub Name, pub Vec<Insert<Value>>);

impl<D: Default> Rewriter<D> for OpTemplate {
  type Output = Id<D>;
    fn rewrite(&self, res: &HashMap<Symbol, MatchValue<D>>, egraph: &mut EGraph<D>) -> Option<Id<D>> {
      let r = self.1.iter().map(|i| i.rewrite(res, egraph)).collect::<Option<Vec<_>>>()?;

      // let form = r.iter().map(|id| id.get_form()).cloned().collect::<Vec<Form>>();
      // let form = Form::Form(o.opcode.clone(), form);

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
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Insert<T> {
  Var(Symbol),
  Use(OpTemplate),
  Lit(T),
}

impl<D: Default> Rewriter<D> for Insert<Value> {
  type Output = Id<D>;
  fn rewrite(&self, res: &HashMap<Symbol, MatchValue<D>>, egraph: &mut EGraph<D>) -> Option<Id<D>> {
    match self {
      Insert::Use(op) => op.rewrite(res, egraph),
      Insert::Var(sym) => Some(egraph.add_node(res.get(sym)?.clone())),
      Insert::Lit(node) => Some(egraph.add_node(node.clone())),
    }
  }
}
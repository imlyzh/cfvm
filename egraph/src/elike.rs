use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
  eclass::{EClass, Id},
  enode::{ENode, RawENode},
};

use fcir::rewriter::form::Form;

#[derive(Debug, Clone)]
pub struct ELike<D>(HashMap<Form, Vec<ENode<D>>>);

impl<D> Default for ELike<D> {
  fn default() -> ELike<D> {
    ELike(HashMap::new())
  }
}

impl<D> ELike<D> {
  pub fn new() -> ELike<D> {
    Default::default()
  }
}

impl<D> ELike<D> {
  pub fn find_collect(&mut self, form: &Form) -> Option<&Vec<ENode<D>>> {
    self.0.get(form)
  }
  pub fn find_collect_mut(&mut self, form: &Form) -> Option<&mut Vec<ENode<D>>> {
    self.0.get_mut(form)
  }
}

impl<D: Default> ELike<D> {
  pub fn add_node(&mut self, form: &Form, node: ENode<D>) -> Id<D> {
    let mut_vector;
    if let Some(nodes) = self.0.get_mut(form) {
      for n in nodes.iter() {
        if *n == node {
          return n.get_id();
        }
      }
      mut_vector = nodes;
    } else {
      self.0.insert(form.clone(), vec![]).unwrap();
      mut_vector = self.0.get_mut(form).unwrap();
    }
    let append_node = node;
    let id = append_node.get_id();
    mut_vector.push(append_node);
    id
  }

  pub fn add_raw_node(&mut self, form: &Form, node: RawENode<D>) -> Id<D> {
    let mut_vector;
    if let Some(nodes) = self.0.get_mut(form) {
      for n in nodes.iter() {
        if n.body.clone() == node {
          return n.get_id();
        }
      }
      mut_vector = nodes;
    } else {
      self.0.insert(form.clone(), vec![]);
      mut_vector = self.0.get_mut(form).unwrap();
    }
    let id = Id(Rc::new_cyclic(|eclass| {
      RefCell::new(EClass::from(ENode {
        eclass: eclass.clone(),
        body: node,
      }))
    }));
    mut_vector.push(id.as_ref().borrow().nodes[0].clone());
    id
  }
  /*
  pub fn add_node(&mut self, form: &Form, node: ENode<D>) {
    if let Some(mut ids) = self.0.get_mut(form).cloned() {
      // 拿到了相似的 eclass 集合
      // Similar eclasses set found
      for id in &ids {
        // 逐个判断是不是一样的，如果是就返回
        // check equal eclass
        if id.as_ref().borrow().find_node(&node) {
          return id.clone();
        }
      }
      // 没找到一样的
      // not equal eclass found
      let id = Id::new(EClass::from(node));
      // 将新建的 eclass 加入相似集
      ids.push(id);
    } else {
      // 没有相似的类的集合
      // No similar eclasses set found
      let id = Id::new(EClass::from(node));
      // 从新建的 eclas s创建新的相似集
      // from eclass create similar set
      self.0.insert(form.clone(), vec![id.clone()]);
      id
    }
  }
  //  */
}

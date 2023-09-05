use std::{cell::RefCell, hash::Hash, rc::Rc};

use crate::enode::{RawENode, ENode};

#[derive(Debug)]
pub struct Id<D>(pub Rc<RefCell<EClass<D>>>); // warning: multi-thread unsound

impl<D> Id<D> {
  pub fn new(value: EClass<D>) -> Self {
    Id(Rc::new(RefCell::new(value)))
  }
}

impl<D> AsRef<RefCell<EClass<D>>> for Id<D> {
  fn as_ref(&self) -> &RefCell<EClass<D>> {
    self.0.as_ref()
  }
}

impl<D> Hash for Id<D> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    Rc::into_raw(self.0.clone()).hash(state);
  }
}

impl<D> PartialEq for Id<D> {
  fn eq(&self, other: &Self) -> bool {
    Rc::into_raw(self.0.clone()) as usize == Rc::into_raw(other.0.clone()) as usize
  }
}

impl<D> Clone for Id<D> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

/*
impl<D> Id<D> {
  pub fn get(&mut self) -> &mut EClass<D> {
    self.0.as_ref()
  }
}
//  */

#[derive(Debug)]
pub struct EClass<D> {
  pub nodes: Vec<ENode<D>>,
  data: D,
}

impl<D: Default> Default for EClass<D> {
  fn default() -> Self {
    Self {
      nodes: Default::default(),
      data: Default::default(),
    }
  }
}

impl<D: Default> From<ENode<D>> for EClass<D> {
  fn from(node: ENode<D>) -> Self {
    let mut this: Self = Default::default();
    this.add_node(node);
    this
  }
}

impl<D> EClass<D> {
  pub fn new(data: D) -> Self {
    Self {
      nodes: Default::default(),
      data,
    }
  }

  pub fn find_node(&self, node: &ENode<D>) -> bool {
    for i in &self.nodes {
      if i == node {
        return true;
      }
    }
    false
  }

  pub fn add_node(&mut self, node: ENode<D>) {
    // if !self.nodes.contains(&node) {
    self.nodes.push(node)
    // }
  }
}

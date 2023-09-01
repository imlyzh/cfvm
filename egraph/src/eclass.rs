use fcir::op::OpHand;

use crate::enode::ENode;

#[derive(Debug, Clone)]
pub struct Id<D>(pub *mut EClass<D>); // warning: multi-thread unsound

#[derive(Debug)]
pub struct EClass<D> {
  nodes: Vec<ENode<D>>,
  data: D,
}

impl<D> EClass<D> {
  pub fn add_node(&mut self, value: ENode<D>) {
    self.nodes.push(value)
  }
}

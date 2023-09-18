use std::{
  cell::RefCell,
  rc::{Rc, Weak},
};

use cfir::{
  block::Region,
  op::Attr,
  rewriter::form::{Form, GetForm},
  symbol::{Name, Symbol},
  types::FuncType,
  value::{Argument, Constant, Label},
};

use crate::eclass::{EClass, Id};

/// EOp from/into Op
#[derive(Debug, Clone)]
pub struct EOp<D> {
  // pub form_cache: RefCell<Option<Form>>,
  pub form_cache: Form,
  pub opcode: Name,
  pub def: Option<Symbol>,
  pub uses: Vec<Id<D>>,
  pub attr: Attr,
  pub region: Region,
  pub sign: FuncType,
}

impl<D> GetForm for EOp<D> {
  fn get_form(&self) -> Option<Form> {
    // if let Some(x) = self.form_cache.borrow().as_ref() {
    //   x.clone()
    // } else {
    //   unreachable!()
    // }
    Some(self.form_cache.clone())
  }
}

impl<D> PartialEq for EOp<D> {
  fn eq(&self, other: &Self) -> bool {
    self.opcode == other.opcode
      && self.uses == other.uses
      && self.attr == other.attr
      && self.region == other.region
      && self.sign == other.sign
  }
}

#[derive(Debug)]
pub struct EOpHand<D>(Rc<RefCell<EOp<D>>>);

impl<D> GetForm for EOpHand<D> {
  fn get_form(&self) -> Option<Form> {
    self.as_ref().borrow().get_form()
  }
}

impl<D> Clone for EOpHand<D> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<D> PartialEq for EOpHand<D> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<D> EOpHand<D> {
  pub fn new(value: EOp<D>) -> Self {
    Self(Rc::new(RefCell::new(value)))
  }
}

impl<D> AsRef<RefCell<EOp<D>>> for EOpHand<D> {
  fn as_ref(&self) -> &RefCell<EOp<D>> {
    self.0.as_ref()
  }
}

#[derive(Debug)]
pub struct ENode<D> {
  pub eclass: Weak<RefCell<EClass<D>>>,
  pub body: RawENode<D>,
}

impl<D> ENode<D> {
  pub fn get_id(&self) -> Id<D> {
    Id(self.eclass.upgrade().unwrap())
  }
}

impl<D> PartialEq for ENode<D> {
  fn eq(&self, other: &Self) -> bool {
    self.body == other.body
  }
}

impl<D> Clone for ENode<D> {
  fn clone(&self) -> Self {
    Self {
      eclass: self.eclass.clone(),
      body: self.body.clone(),
    }
  }
}

impl<D> GetForm for ENode<D> {
  fn get_form(&self) -> Option<Form> {
    self.body.get_form()
  }
}

/// RawENode from/into Value
#[derive(Debug)]
pub enum RawENode<D> {
  Const(Constant),
  Use(EOpHand<D>),
  Argument(Argument),
  Label(Label),
  Input(Symbol),
}

impl<D> Clone for RawENode<D> {
  fn clone(&self) -> Self {
    match self {
      Self::Const(arg0) => Self::Const(arg0.clone()),
      Self::Use(arg0) => Self::Use(arg0.clone()),
      Self::Argument(arg0) => Self::Argument(arg0.clone()),
      Self::Label(arg0) => Self::Label(arg0.clone()),
      Self::Input(arg0) => Self::Input(arg0.clone()),
    }
  }
}

impl<D> GetForm for RawENode<D> {
  fn get_form(&self) -> Option<Form> {
    match self {
      RawENode::Use(op) => op.get_form(),
      RawENode::Const(_) | RawENode::Argument(_) | RawENode::Label(_) | RawENode::Input(_) => {
        Some(Form::Atom)
      },
    }
  }
}

impl<D> PartialEq for RawENode<D> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Const(l0), Self::Const(r0)) => l0 == r0,
      (Self::Use(l0), Self::Use(r0)) => l0 == r0,
      (Self::Argument(l0), Self::Argument(r0)) => l0 == r0,
      (Self::Label(l0), Self::Label(r0)) => l0 == r0,
      _ => false,
    }
  }
}

impl<D> From<&Constant> for RawENode<D> {
  fn from(value: &Constant) -> Self {
    RawENode::Const(value.clone())
  }
}

impl<D> From<&Argument> for RawENode<D> {
  fn from(value: &Argument) -> Self {
    RawENode::Argument(value.clone())
  }
}

impl<D> From<&Label> for RawENode<D> {
  fn from(value: &Label) -> Self {
    RawENode::Label(value.clone())
  }
}

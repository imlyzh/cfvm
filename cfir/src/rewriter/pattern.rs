use std::{cell::RefCell, hash::Hash, rc::Rc};

use crate::{
  symbol::{Name, Symbol},
  value::{Argument, Constant, Label},
};

use super::form::{Form, GetForm};

pub trait Matcher<T> {
  type Output;
  fn matching(&self, i: &T) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpPat(
  // pub Catch<Name>,
  pub Name,
  pub Vec<Catch<ValuePat>>,
  pub Vec<Symbol>,
  // pub FuncType,
);

impl GetForm for OpPat {
  fn get_form(&self) -> Option<Form> {
    Some(Form::Form(
      // self.0 .0.clone(),
      self.0.clone(),
      self.1.iter().map(GetForm::get_form).collect(),
    ))
  }
}

#[derive(Debug, Clone, Eq)]
pub struct OpPatHand(Rc<RefCell<OpPat>>);

impl OpPatHand {
  pub fn new(value: OpPat) -> Self {
    Self(Rc::new(RefCell::new(value)))
  }
}

impl GetForm for OpPatHand {
  fn get_form(&self) -> Option<Form> {
    self.as_ref().borrow().get_form()
  }
}

impl AsRef<RefCell<OpPat>> for OpPatHand {
  fn as_ref(&self) -> &RefCell<OpPat> {
    self.0.as_ref()
  }
}

impl Hash for OpPatHand {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    Rc::into_raw(self.0.clone()).hash(state);
  }
}

impl PartialEq for OpPatHand {
  fn eq(&self, other: &Self) -> bool {
    Rc::into_raw(self.0.clone()) as usize == Rc::into_raw(other.0.clone()) as usize
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Catch<T>(pub Option<T>, pub Option<Symbol>);

impl<T: GetForm> GetForm for Catch<T> {
  fn get_form(&self) -> Option<Form> {
    self.0.get_form()
  }
}

impl<T: GetForm> GetForm for Option<T> {
  fn get_form(&self) -> Option<Form> {
    if let Some(x) = self {
      x.get_form()
    } else {
      None
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValuePat {
  Const(Constant),
  // Use(OpPatHand),
  Use(OpPatHand, usize),
  Argument(Argument),
  Label(Label),
  Input(Symbol),
}

impl GetForm for ValuePat {
  fn get_form(&self) -> Option<Form> {
    match self {
      ValuePat::Use(op, 0) => op.get_form(),
      ValuePat::Use(_op, _) => None,
      ValuePat::Const(_) | ValuePat::Argument(_) | ValuePat::Label(_) | ValuePat::Input(_) => {
        Some(Form::Atom)
      },
    }
  }
}

impl Matcher<Name> for Name {
  type Output = Option<()>;
  fn matching(&self, i: &Name) -> Self::Output {
    if self == i {
      Some(())
    } else {
      None
    }
  }
}

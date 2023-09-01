use crate::{symbol::Symbol, op::Attr, value::Constant};


#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Form {
  Atom,
  /// Form(opname, args, metadata, region)
  Form(Symbol, Vec<Form>, Vec<(Symbol, Constant)>)
}

pub trait GetForm {
  fn get_form(&self) -> Form;
}


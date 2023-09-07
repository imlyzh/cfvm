use crate::symbol::Name;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Form {
  Atom,
  /// TODO: Form(opcode, args, metadata, region)
  // Form(Symbol, Vec<Form>, Vec<(Symbol, Constant)>)
  /// Form(opcode, args)
  Form(Name, Vec<Form>),
}

pub trait GetForm {
  fn get_form(&self) -> Form;
}

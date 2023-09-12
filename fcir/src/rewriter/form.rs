use crate::symbol::Name;

#[derive(Debug, Clone, Eq)]
pub enum Form {
  Atom,
  /// TODO: Form(opcode, args, metadata, region)
  // Form(Symbol, Vec<Form>, Vec<(Symbol, Constant)>)
  /// Form(opcode, args)
  Form(Name, Vec<Option<Form>>),
}

impl std::hash::Hash for Form {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      Form::Atom => 0_u8.hash(state),
      Form::Form(_name, arg) => {
        // name.hash(state);
        arg.len().hash(state);
      },
    }
  }
}

impl PartialEq for Form {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Form(l0, l1), Self::Form(r0, r1)) => {
        l0 == r0 && {
          for i in l1.iter().zip(r1.iter()) {
            match i {
              (Some(l), Some(r)) => {
                if l != r {
                  return false;
                }
              },
              (None, None) | (None, Some(_)) | (Some(_), None) => {},
            }
          }
          true
        }
      },
      (Self::Atom, Self::Atom) => true,
      _ => false,
    }
  }
}

pub trait GetForm {
  fn get_form(&self) -> Option<Form>;
}

use std::num::ParseFloatError;

use sexpr_ir::{gast::GAst};

use crate::nodes::Module;



trait FromGast {
    type Target;
    fn from_gast(&self, ast: &GAst) -> Result<Self::Target, ()>;
}

impl FromGast for Module {
    type Target = Self;

    fn from_gast(&self, ast: &GAst) -> Result<Self::Target, ()> {
        todo!()
    }
}
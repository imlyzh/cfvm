
// partial evaluation


use std::sync::{RwLock, Arc};

use crate::pass::richir::Context;

use crate::cfir::richir::{LetBinding, Expr, Value};


trait Pe {
    fn pe(&self, ctx: Context) -> Self;
}

impl Pe for LetBinding {
    fn pe(&self, ctx: Context) -> Self {
        todo!()
    }
}

impl Pe for Expr {
    fn pe(&self, ctx: Context) -> Self {
        todo!()
    }
}

impl Pe for Value {
    fn pe(&self, ctx: Context) -> Self {
        todo!()
    }
}
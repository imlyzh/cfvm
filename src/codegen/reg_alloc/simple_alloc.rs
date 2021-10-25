use std::collections::HashMap;

use crate::{cfir::graphir::{FunctionDef, basicblock::BasicBlockDef}, codegen::InstStream};

use super::RegConfig;



pub trait SimpleAlloc {
    fn simple_alloc(self, rcfg: RegConfig, bs: &mut BasicBlockDef, fd: &mut FunctionDef) -> InstStream;
}

impl SimpleAlloc for InstStream {
    fn simple_alloc(self, rcfg: RegConfig, bs: &mut BasicBlockDef, fd: &mut FunctionDef) -> InstStream {

        todo!()
    }}
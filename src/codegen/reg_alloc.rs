use std::ops::Range;

use crate::cfir::graphir::{FunctionDef, basicblock::BasicBlockDef, handles::{LazyLoadSymbol, LocalSymbol, SymbolHandle}, instruction::{BindOperator, Instruction, Store}};


#[derive(Debug, Clone)]
pub struct InstStream(pub Vec<Instruction>);

pub trait Streamify {
    fn streamify(self) -> InstStream;
}

impl Streamify for BasicBlockDef {
    fn streamify(self) -> InstStream {
        InstStream(self.instructions.read().unwrap().iter().map(|x| x.read().unwrap().clone()).collect())
    }
}


#[derive(Debug, Clone, Copy)]
pub struct RegConfig {
    pub reg_count: usize,
    pub exreg_count: usize,
}

pub trait LinealScanAlloc {
    fn linear_scan_alloc(self, rcfg: RegConfig, bs: &mut BasicBlockDef, fd: &mut FunctionDef) -> InstStream;
}

pub type Lifetime = (Vec<(usize, usize)>, Vec<(LocalSymbol, usize)>);

pub fn find_inst_var_lifetime(
    i: &Instruction,
    inst_num: usize,
    bs: &mut BasicBlockDef,
    fd: &mut FunctionDef
) -> Lifetime {
    let mut reg_lifetime: Vec<(usize, usize)> = vec![];
    let mut var_lifetime: Vec<(LocalSymbol, usize)> = vec![];
    match i {
        Instruction::Store(Store(v, e, _, _))
            => {
                let r = v.0.read().unwrap().clone();
                match r {
                    LazyLoadSymbol::Symbol(v)
                        => var_lifetime.push((v.clone(), inst_num*3+1)),
                    LazyLoadSymbol::Reference(v) => {
                        let (rr, rv) = find_inst_var_lifetime(i, 0, bs, fd);
                        todo!()
                        },
                }
            }
        Instruction::BindOperator(BindOperator(v, e, _, _)) => {
                var_lifetime.push((v.clone(), inst_num+2));
                todo!()
            }
        Instruction::Operator(e)=>{
            todo!()
        }
    }
    (reg_lifetime, var_lifetime)
}

impl LinealScanAlloc for InstStream {
    fn linear_scan_alloc(self, rcfg: RegConfig, bs: &mut BasicBlockDef, fd: &mut FunctionDef) -> InstStream {
        let InstStream(insts) = self;
        let mut reg_set = vec![false; rcfg.reg_count*3];
        let mut exreg_set = vec![false; rcfg.exreg_count*3];
        let mut reg_lifetime: Vec<(usize, usize)> = vec![];
        let mut var_lifetime: Vec<(LocalSymbol, usize)> = vec![];
        for (inst_num, i) in insts.iter().enumerate() {
            let (ret_reg_lifetime, ret_var_lifetime, ) = find_inst_var_lifetime(i, inst_num, bs, fd);
        }
        todo!()
    }
}
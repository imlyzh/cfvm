
use std::{collections::{BinaryHeap, HashMap, HashSet}, ops::Range};

use crate::{analysis::find_lifetime::FindVarLifetime, cfir::graphir::{FunctionDef, basicblock::BasicBlockDef, handles::{LazyLoadSymbol, LocalSymbol, SymbolHandle}, instruction::{BindOperator, Instruction, Store}}, codegen::InstStream};

use super::RegConfig;

pub trait LinealScanAlloc {
    fn linear_scan_alloc(self, rcfg: RegConfig, bs: &mut BasicBlockDef, fd: &mut FunctionDef) -> InstStream;
}


impl LinealScanAlloc for InstStream {
    fn linear_scan_alloc(self, rcfg: RegConfig, bs: &mut BasicBlockDef, fd: &mut FunctionDef) -> InstStream {
        let InstStream(insts) = self;
        let mut reg_lifetime: Vec<(usize, usize)> = vec![];
        let mut var_lifetime: Vec<(LocalSymbol, usize)> = vec![];
        for (inst_num, i) in insts.iter().enumerate() {
            i.find_var_lifetime(true, inst_num, &mut reg_lifetime, &mut var_lifetime);
        }
        let reg_set: HashSet<usize> = reg_lifetime.iter().map(|(x, _)| x).cloned().collect();
        let var_set: HashSet<LocalSymbol> = var_lifetime.iter().map(|(x, _)| x).cloned().collect
        ();
        let buffer = vec![false; rcfg.reg_count*3];
        let exbuffer = vec![false; rcfg.exreg_count*3];
        todo!()
    }
}

/*
        let reg_map: Vec<(usize, BinaryHeap<usize>)> = reg_set
            .iter()
            .map(|x| (x.clone(),
                reg_lifetime
                    .iter()
                    .filter(|(y, _)| y == x)
                    .map(|(_, z)| *z)
                    .collect::<BinaryHeap<_>>()))
            .collect();
        let var_map: Vec<(LocalSymbol, BinaryHeap<usize>)> = var_set
            .iter()
            .map(|x| (x.clone(),
                var_lifetime
                    .iter()
                    .filter(|(y, _)| y == x)
                    .map(|(_, z)| *z)
                    .collect::<BinaryHeap<_>>()))
            .collect();
         */
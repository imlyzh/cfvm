use libemei::insts::inst_dump_buf::InstBuffer;

use crate::{analysis::get_symbol::GetSymbol, cfir::graphir::{handles::{LazyLoadSymbol, LocalHandle, LocalSymbol}, instruction::{BindOperator, Instruction, Operator, Store}}};

use super::{CodeGen, InstStream};


impl CodeGen for InstStream {
    fn gen_code(&self, buf: &InstBuffer) {
        for i in self.0.iter() {
            inst_gen_code(i, buf);
        }
    }
}


fn inst_gen_code(inst: &Instruction, buf: &InstBuffer) {
    match inst {
        Instruction::Store(Store(s, e, a, v)) => {
            let s = s.get_symbol();

        },
        Instruction::BindOperator(BindOperator(l, o, a, v)) => {
            let s = l.get_symbol();
        },
        Instruction::Operator(op) => {
            match op.read().unwrap().clone() {
                Operator::SetValue(o, i, v) => {

                },
                Operator::SetItem(o, i, v) => {
                    todo!()
                },
                Operator::Call(f, args) => {
                    todo!()
                },
                _ => {
                    panic!("unsupported operator")
                },
            }
        },
    }
}

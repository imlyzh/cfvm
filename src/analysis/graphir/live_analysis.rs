use std::collections::HashMap;

use crate::cfir::{
    graphir::{BasicBlockDef, FunctionDef, instruction::{Instruction, Operator, BindOperator, Store}},
    handles::{LocalSymbol, LabelSymbol, SymbolRef}
};

use super::{
    Analysis,
    RootAnalysis,
    untils::get_all_veriable,
};


pub type FunLiveVar = HashMap<LabelSymbol, BBSLiveVar>;
pub type BBSLiveVar = HashMap<LocalSymbol, bool>;


impl RootAnalysis for FunctionDef {
    type Output = FunLiveVar;

    fn live_analysis(&self) -> Self::Output {
        let vars: BBSLiveVar =
            get_all_veriable(self)
                .into_iter()
                .map(|x| (x, false))
                .collect();
        let vars: HashMap<LabelSymbol, BBSLiveVar> = self.bbs.borrow().iter()
            .map(|x| (x.borrow().label.clone(), vars.clone()))
            .collect();
        todo!()
    }

}

impl Analysis for BasicBlockDef {
    type Context = BBSLiveVar;

    fn live_analysis(&self, mut record: Self::Context) -> Self::Context {
        for i in self.instructions.borrow().iter() {
            use_variable_for_insts(&i.borrow().to_owned(), &mut record);
        }
        record
    }
}

pub fn use_variable_for_insts(inst: &Instruction, record: &mut BBSLiveVar) {
    match inst {
        Instruction::Store(Store(v, v1, _ty)) => {
            record.insert(v.clone(), true);
            if let SymbolRef::Local(v1) = v1 {
                record.insert(v1.clone(), true);
            }
        },
        Instruction::BindOperator(BindOperator(_, oper)) =>
            use_variable_for_opers(&oper.borrow(), record),
        Instruction::Operator(oper) =>
            use_variable_for_opers(&oper.borrow(), record),
    }
}

pub fn use_variable_for_opers(oper: &Operator, record: &mut BBSLiveVar) {
    match oper {
        Operator::Alloca(_, _) => todo!(),
        Operator::GetPtr(_, _) => todo!(),
        Operator::Load(_, _) => todo!(),
        Operator::Cast(_, _) => todo!(),
        Operator::Add(_, _) => todo!(),
        Operator::FAdd(_, _) => todo!(),
        Operator::Sub(_, _) => todo!(),
        Operator::FSub(_, _) => todo!(),
        Operator::Mul(_, _) => todo!(),
        Operator::FMul(_, _) => todo!(),
        Operator::UDiv(_, _) => todo!(),
        Operator::SDiv(_, _) => todo!(),
        Operator::URem(_, _) => todo!(),
        Operator::SRem(_, _) => todo!(),
        Operator::FRem(_, _) => todo!(),
        Operator::Shl(_, _) => todo!(),
        Operator::LShr(_, _) => todo!(),
        Operator::AShr(_, _) => todo!(),
        Operator::And(_, _) => todo!(),
        Operator::Or(_, _) => todo!(),
        Operator::Xor(_, _) => todo!(),
        Operator::GetValue(_, _) => todo!(),
        Operator::GetItem(_, _) => todo!(),
        Operator::SetValue(_, _, _) => todo!(),
        Operator::SetItem(_, _, _) => todo!(),
        Operator::Trunc(_, _) => todo!(),
        Operator::ZExt(_, _) => todo!(),
        Operator::SExt(_, _) => todo!(),
        Operator::FTrunc(_, _) => todo!(),
        Operator::FExt(_, _) => todo!(),
        Operator::ICmp(_, _, _) => todo!(),
        Operator::FCmp(_, _, _) => todo!(),
        Operator::Phi(_) => todo!(),
        Operator::Call(_, _) => todo!(),
    }
}
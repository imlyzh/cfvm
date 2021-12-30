use std::collections::{HashMap, VecDeque, HashSet};

use crate::cfir::{
    graphir::{BasicBlockDef, FunctionDef, instruction::{Instruction, Operator, BindOperator, Store, Terminator, Conds, Branch, Switch, Ret}, Value},
    handles::{LocalSymbol, LabelSymbol, SymbolRef}
};

use super::untils::get_all_veriable;


pub type FunLiveVar = HashMap<LabelSymbol, BBSLiveVar>;
pub type BBSLiveVar = HashMap<LocalSymbol, bool>;

pub trait RootLiveAnalysis {
    fn live_analysis(&self) -> FunLiveVar;
}

pub trait LiveAnalysis {
    fn live_analysis(&self, record: BBSLiveVar) -> BBSLiveVar;
}

impl RootLiveAnalysis for FunctionDef {
    fn live_analysis(&self) -> FunLiveVar {
        let vars: BBSLiveVar =
            get_all_veriable(self)
                .into_iter()
                .map(|x| (x, false))
                .collect();
        let mut old: FunLiveVar = self.bbs.borrow().iter()
            .map(|x| (x.borrow().label.clone(), vars.clone()))
            .collect();
        loop {
            let new = once_pass(self, old.clone());
            if old == new {
                break;
            } else {
                old = new;
            }
        }
        old
    }

}

pub fn once_pass(fun_def: &FunctionDef, mut inp: FunLiveVar) -> FunLiveVar {
    dbg!("once_pass");
    let mapping = fun_def.bbs.borrow().iter().enumerate()
        .map(|(offset, x)| (x.borrow().label.clone(), offset)).collect::<HashMap<_, _>>();
    let mut next_set: VecDeque<LabelSymbol> = VecDeque::new();
    let mut used_bb: HashSet<LabelSymbol> = HashSet::new();
    next_set.push_back(fun_def.bbs.borrow()[0].borrow().label.clone());
    while !next_set.is_empty() {
        let task = next_set.pop_front().unwrap();
        if !used_bb.contains(&task) {
            let bb_offset = mapping.get(&task).unwrap();
            let bb = &fun_def.bbs.borrow()[*bb_offset];
            inp.insert(task.clone(), bb.borrow().live_analysis(inp.get(&task).unwrap().clone()));
            if let Some(x) = bb.borrow().get_next() {
                for next in x.iter() {
                    next_set.push_back(next.clone().clone());
                }
                if x.is_empty() && bb_offset + 1 < fun_def.bbs.borrow().len() {
                    next_set.push_back(fun_def.bbs.borrow()[bb_offset + 1].borrow().label.clone());
                }
            } else {
                return inp;
            }
            used_bb.insert(task);
        }
    }
    inp
}

impl LiveAnalysis for BasicBlockDef {
    fn live_analysis(&self, mut record: BBSLiveVar) -> BBSLiveVar {
        dbg!("live_analysis");
        for i in self.instructions.borrow().iter() {
            use_variable_for_insts(&i.borrow().to_owned(), &mut record);
        }
        if let Some(x) = &self.terminator {
            use_variable_for_terminator(&x.borrow(), &mut record);
        }
        record
    }
}

pub fn use_variable_for_terminator(ter: &Terminator, record: &mut BBSLiveVar) {
    match ter {
        Terminator::Branch(Branch(_, v, _, _)) |
        Terminator::Switch(Switch(v, _)) => use_variable_for_symbolref(&v, record),
        Terminator::Ret(Ret(v)) => {
            if let Some(Value::Var(v)) = v {
                use_variable_for_symbolref(&v, record);
            }
        },
        Terminator::Conds(Conds(cs, _)) => {
            for (v, _) in cs {
                use_variable_for_symbolref(&v, record);
            }
        },
        Terminator::Unrechable => {},
    }
}

pub fn use_variable_for_insts(inst: &Instruction, record: &mut BBSLiveVar) {
    match inst {
        Instruction::Store(Store(v, v1, _ty)) => {
            record.insert(v.clone(), true);
            use_variable_for_symbolref(v1, record);
        },
        Instruction::BindOperator(BindOperator(_, oper)) =>
            use_variable_for_opers(&oper.borrow(), record),
        Instruction::Operator(oper) =>
            use_variable_for_opers(&oper.borrow(), record),
    }
}

pub fn use_variable_for_opers(oper: &Operator, record: &mut BBSLiveVar) {
    match oper {
        Operator::Alloca(_, _v) => {},
        Operator::GetPtr(v, _)  |
        Operator::Load(_, v)    |
        Operator::Cast(_, v)    |
        Operator::Trunc(v, _)   |
        Operator::ZExt(v, _)    |
        Operator::SExt(v, _)    |
        Operator::GetValue(v, _)|
        Operator::FTrunc(v, _)  |
        Operator::FExt(v, _)    => use_variable_for_symbolref(v, record),
        Operator::Add(v0, v1)       |
        Operator::FAdd(v0, v1)      |
        Operator::Sub(v0, v1)       |
        Operator::FSub(v0, v1)      |
        Operator::Mul(v0, v1)       |
        Operator::FMul(v0, v1)      |
        Operator::UDiv(v0, v1)      |
        Operator::SDiv(v0, v1)      |
        Operator::URem(v0, v1)      |
        Operator::SRem(v0, v1)      |
        Operator::FRem(v0, v1)      |
        Operator::Shl(v0, v1)       |
        Operator::LShr(v0, v1)      |
        Operator::AShr(v0, v1)      |
        Operator::And(v0, v1)       |
        Operator::Or(v0, v1)        |
        Operator::Xor(v0, v1)       |
        Operator::ICmp(_, v0, v1)   |
        Operator::FCmp(_, v0, v1)   |
        Operator::GetItem(v0, v1)   |
        Operator::SetValue(v0, _, v1) => {
            use_variable_for_symbolref(v0, record);
            use_variable_for_symbolref(v1, record);
        }
        Operator::SetItem(v0, v1, v2) => {
            use_variable_for_symbolref(v0, record);
            use_variable_for_symbolref(v1, record);
            use_variable_for_symbolref(v2, record);
        },
        Operator::Phi(vn) => {
            for (_, v) in vn.iter() {
                use_variable_for_symbolref(v, record);
            }
        },
        Operator::Call(v0, vn) => {
            use_variable_for_symbolref(v0, record);
            for v in vn.iter() {
                use_variable_for_symbolref(v, record);
            }
        },
    }
}

pub fn use_variable_for_symbolref(v: &SymbolRef, record: &mut BBSLiveVar) {
    if let SymbolRef::Local(v) = v {
        record.insert(v.clone(), true);
    }
}
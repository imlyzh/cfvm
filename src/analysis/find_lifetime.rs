use crate::cfir::graphir::{handles::{LazyLoadSymbol, LocalSymbol, SymbolRef, ValueHandle}, instruction::{BindOperator, Instruction, Operator, Store}};



// pub type Lifetime = (Vec<(usize, usize)>, Vec<(LocalSymbol, usize)>);

pub trait FindVarLifetime {
    fn find_var_lifetime(&self,
        deep: bool,
        inst_num: usize,
        reg_lifetime: &mut Vec<(usize, usize)>,
        var_lifetime: &mut Vec<(LocalSymbol, usize)>
    );
}

impl FindVarLifetime for ValueHandle {
    fn find_var_lifetime(&self,
        deep: bool,
        inst_num: usize,
        reg_lifetime: &mut Vec<(usize, usize)>,
        var_lifetime: &mut Vec<(LocalSymbol, usize)>) {
        match self.0.read().unwrap().clone() {
            LazyLoadSymbol::Symbol(SymbolRef::Local(s)) => todo!(),
            LazyLoadSymbol::Reference(r) => todo!(),
            _ => {}
        }
        todo!()
    }
}

impl FindVarLifetime for Operator {
    fn find_var_lifetime(&self,
        deep: bool,
        inst_num: usize,
        reg_lifetime: &mut Vec<(usize, usize)>,
        var_lifetime: &mut Vec<(LocalSymbol, usize)>) {
        match self {
            // 0
            Operator::Alloca(_at, _t) => {},
            // 1
            Operator::GetPtr(a, _)      |
            Operator::Load(_,a)         |
            Operator::Cast(_, a)        |
            Operator::GetValue(a, _)    |
            Operator::Trunc(a, _)       |
            Operator::ZExt(a, _)        |
            Operator::SExt(a, _)        |
            Operator::FTrunc(a, _)      |
            Operator::FExt(a, _)        => {
                todo!()
            }
            // 2
            Operator::Add(a, b)         |
            Operator::FAdd(a, b)        |
            Operator::Sub(a, b)         |
            Operator::FSub(a, b)        |
            Operator::Mul(a, b)         |
            Operator::FMul(a, b)        |
            Operator::UDiv(a, b)        |
            Operator::SDiv(a, b)        |
            Operator::URem(a, b)        |
            Operator::SRem(a, b)        |
            Operator::FRem(a, b)        |
            Operator::Shl(a, b)         |
            Operator::LShr(a, b)        |
            Operator::AShr(a, b)        |
            Operator::And(a, b)         |
            Operator::Or(a, b)          |
            Operator::Xor(a, b)         |
            Operator::GetItem(a, b)     |
            Operator::SetValue(a, _, b) |
            Operator::SetItem(a, _, b)  |
            Operator::ICmp(_, a, b)     |
            Operator::FCmp(_, a, b)     => {
                todo!()
            }
            // n
            Operator::Phi(nodes) => {
                todo!()
            }
            Operator::Call(a, bs) => {
                todo!()
            }
        }
        todo!()
    }
}


impl FindVarLifetime for Instruction {
    fn find_var_lifetime(&self,
        deep: bool,
        inst_num: usize,
        reg_lifetime: &mut Vec<(usize, usize)>,
        var_lifetime: &mut Vec<(LocalSymbol, usize)>) {
        match self {
            Instruction::Store(Store(v, e, _, _))
                => {
                    match v.0.read().unwrap().clone() {
                        LazyLoadSymbol::Symbol(v)
                            => var_lifetime.push((v, inst_num*2)),
                        LazyLoadSymbol::Reference(v) => {
                            v.find_var_lifetime(false, inst_num, reg_lifetime, var_lifetime);
                            todo!()
                            },
                    }
                    if deep {
                        todo!()
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
    }
}

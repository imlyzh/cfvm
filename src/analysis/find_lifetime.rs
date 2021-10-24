use crate::cfir::graphir::{handles::{LazyLoadSymbol, LocalSymbol, SymbolHandle, SymbolRef, ValueHandle, ValueRef}, instruction::{BindOperator, Instruction, Operator, Store}};



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
            LazyLoadSymbol::Symbol(SymbolRef::Local(s)) => var_lifetime.push((s, inst_num*2)),
            LazyLoadSymbol::Reference(ValueRef::Local(r)) => r.find_var_lifetime(deep, inst_num, reg_lifetime, var_lifetime),
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
                a.find_var_lifetime(deep, inst_num, reg_lifetime, var_lifetime)
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
                a.find_var_lifetime(deep, inst_num, reg_lifetime, var_lifetime);
                b.find_var_lifetime(deep, inst_num, reg_lifetime, var_lifetime)
            }
            // n
            Operator::Phi(nodes) => {
                nodes
                    .iter()
                    .for_each(|(_, x)|
                        x.find_var_lifetime(deep, inst_num, reg_lifetime, var_lifetime));
            }
            Operator::Call(a, bs) => {
                a.find_var_lifetime(deep, inst_num, reg_lifetime, var_lifetime);
                bs
                    .iter()
                    .for_each(|x|
                        x.find_var_lifetime(deep, inst_num, reg_lifetime, var_lifetime));
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
                            },
                    }
                    if deep {
                        e.find_var_lifetime(false, inst_num, reg_lifetime, var_lifetime);
                    }
                }
            Instruction::BindOperator(BindOperator(v, e, _, _)) => {
                if deep {
                    var_lifetime.push((v.clone(), inst_num*2+1));
                } else {
                    var_lifetime.push((v.clone(), inst_num*2));
                }
                if deep {
                    e.read().unwrap().find_var_lifetime(false, inst_num, reg_lifetime, var_lifetime);
                }
                }
            Instruction::Operator(e)=> {
                // e.read().unwrap().find_var_lifetime(false, inst_num, reg_lifetime, var_lifetime),
                }
        }
    }
}

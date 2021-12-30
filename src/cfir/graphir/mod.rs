pub mod instruction;

pub mod parser;

use self::instruction::{Instruction, Terminator, Branch, Conds, Switch};

use super::{
    base::{
        // ConstantDef, VariableDef, TypeDef,
        Module
    },
    types::{
        Type, FunctionType, PointerType, FirstClassType, SimpleType, GetType,
        FunctionAttr
    },
    handles::{DefineSymbol, LabelSymbol, ConstantValue, SymbolRef, LTMHand}
};

pub type GraphModule = Module<FunctionDef>;

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: DefineSymbol,
    pub header: FunctionType,
    pub function_attr: FunctionAttr,
    pub bbs: LTMHand<Vec<LTMHand<BasicBlockDef>>>,
    // pub bbs_map: LTMHand<HashMap<LabelSymbol, usize>>,
}

#[derive(Debug, Clone)]
pub struct BasicBlockDef {
    pub label: LabelSymbol,
    // pub prev_block: LTMHand<Vec<LTMHand<BasicBlockDef>>>,
    // pub variable_defs: LTMHand<HashMap<LabelSymbol, LTMHand<BindOperator>>>,
    pub instructions: LTMHand<Vec<LTMHand<Instruction>>>,
    pub terminator: Option<LTMHand<Terminator>>,
}

impl BasicBlockDef {
    pub fn get_next(&self) -> Option<Vec<LabelSymbol>> {
        self.terminator.as_ref().map_or(Some(vec![]), |x|
            match x.borrow().to_owned() {
                Terminator::Branch(Branch(_, _, t, e)) =>
                    Some(vec![t, e]),
                Terminator::Conds(Conds(bs, e)) =>{
                    let mut r: Vec<_> = bs.iter().map(|x| x.1.clone()).collect();
                    if let Some(x) = e {
                        r.push(x);
                    }
                    Some(r)
                },
                Terminator::Switch(Switch(_, bs)) =>
                    Some(bs.iter().map(|x| x.1.clone()).collect()),
                Terminator::Unrechable |
                Terminator::Ret(_) => None,
            })
    }
}

impl GetType for FunctionDef {
    fn get_type(&self) -> Type {
        let func_type = Type::FunType(self.header.clone());
        // let r = func_type;
        Type::FCType(FirstClassType::SimpleType(SimpleType::Pointer(
            PointerType(Box::new(func_type)),
        )))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Var(SymbolRef),
    Lit(ConstantValue),
}
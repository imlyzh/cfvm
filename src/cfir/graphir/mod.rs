pub mod instruction;

pub mod parser;

use std::{fmt::{Display, Debug}, collections::{HashSet, VecDeque, HashMap}};

use tracing::{debug, instrument};

use self::instruction::{Instruction, Terminator, Branch, Conds, Switch, BindOperator, Operator};

use super::{
    base::{
        // ConstantDef, VariableDef, TypeDef,
        Env
    },
    types::{
        Type, FunctionType, PointerType, FirstClassType, SimpleType, GetType,
        FunctionAttr
    },
    handles::{DefineSymbol, LabelSymbol, ConstantValue, SymbolRef, GlobalSymbol}
};

pub type GraphModule = Env<FunctionDef>;

impl GraphModule {
    /// make_call_graph
    /// Low accuracy call graph(Does not include indirect calls).
    pub fn make_call_graph(&self) -> Vec<(GlobalSymbol, SymbolRef)> {
        let root = self.function_defs
            .iter()
            .filter(|(_, f)| f.function_attr.is_extern.0)
            .map(|(k, _)| GlobalSymbol(k.clone()));
        let mut used_fns: HashSet<GlobalSymbol> = HashSet::new();
        let mut next_set: VecDeque<GlobalSymbol> = VecDeque::new();
        let mut r = Vec::new();
        next_set.extend(root);
        while !next_set.is_empty() {
            let task = next_set.pop_front().unwrap();
            if !used_fns.contains(&task) {
                let fun = self.function_defs.get(&task.0).unwrap();
                for i in fun.get_call_targets() {
                    r.push((task.clone(), i.clone()));
                    if let SymbolRef::Global(x) = i {
                        next_set.push_back(x);
                    }
                }
                used_fns.insert(task);
            }
        }
        r
    }
}

#[derive(Clone)]
pub struct FunctionDef {
    pub name: DefineSymbol,
    pub header: FunctionType,
    pub function_attr: FunctionAttr,
    pub bbs: Vec<BasicBlockDef>,
    // pub bbs_map: LTMHand<HashMap<LabelSymbol, usize>>,
}

impl FunctionDef {
    pub fn make_control_flow_graph(&self) -> Vec<(LabelSymbol, LabelSymbol)> {
        let bbs = &self.bbs;
        let mapping = bbs.iter().enumerate()
            .map(|(offset, x)| (x.label.clone(), offset)).collect::<HashMap<_, _>>();
        let mut used_bbs: HashSet<LabelSymbol> = HashSet::new();
        let mut next_set: VecDeque<LabelSymbol> = VecDeque::new();
        let mut r = Vec::new();
        next_set.push_back(self.bbs[0].label.clone());
        while !next_set.is_empty() {
            let task = next_set.pop_front().unwrap();
            if !used_bbs.contains(&task) {
                let bb_offset = mapping.get(&task).unwrap();
                let bb = &bbs[*bb_offset];
                if let Some(x) = bb.get_next() {
                    if x.is_empty() && bb_offset + 1 < bbs.len() {
                        let target = bbs[bb_offset + 1].label.clone();
                        next_set.push_back(target.clone());
                        r.push((task.clone(), target));
                    } else {
                        for i in x {
                            next_set.push_back(i.clone());
                            r.push((task.clone(), i));
                        }
                    }
                }
                used_bbs.insert(task);
            }
        }
        r
    }

    pub fn get_call_targets(&self) -> HashSet<SymbolRef> {
        let bbs: &Vec<_> = &self.bbs;
        bbs.iter()
            .flat_map(|x| x.get_call_targets())
            .collect()
    }
}

impl Debug for FunctionDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<Fun {}>", (self.name.0).0)
    }
}

impl Display for FunctionDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<Fun {}>", (self.name.0).0)
    }
}

#[derive(Clone)]
pub struct BasicBlockDef {
    pub label: LabelSymbol,
    // pub prev_block: Vec<BasicBlockDef>>,
    // pub variable_defs: HashMap<LabelSymbol, BindOperator>>,
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Terminator>,
}

impl Debug for BasicBlockDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<BasicBlock {}>", (self.label.0).0)
    }
}

impl Display for BasicBlockDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<BasicBlock {}>", (self.label.0).0)
    }
}

impl BasicBlockDef {
    #[instrument(level = "debug")]
    pub fn get_next(&self) -> Option<Vec<LabelSymbol>> {
        let r: Option<Vec<LabelSymbol>> = self.terminator.as_ref().map_or(Some(vec![]), |x|
            match x {
                Terminator::Branch(Branch(_, _, t, e)) =>
                    Some(vec![t.clone(), e.clone()]),
                Terminator::Conds(Conds(bs, e)) =>{
                    let mut r: Vec<LabelSymbol> = bs.iter().map(|x| x.1.clone()).collect();
                    if let Some(x) = e {
                        r.push(x.clone());
                    }
                    Some(r)
                },
                Terminator::Switch(Switch(_, bs)) =>
                    Some(bs.iter().map(|x| x.1.clone()).collect()),
                Terminator::Unrechable |
                Terminator::Ret(_) => None,
            });
            debug!("get_next: {:?}", r);
            r
    }
    fn get_call_targets(&self) -> HashSet<SymbolRef> {
        let is = &self.instructions;
        let mut r = HashSet::new();
        for i in is {
            match i {
                Instruction::BindOperator(BindOperator(_, op)) |
                Instruction::Operator(op) => {
                    let op: &Operator = &op.borrow();
                    if let Operator::Call(f, _) = op {
                        r.insert(f.clone());
                    }
                },
                _ => {}
            }
        }
        r
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
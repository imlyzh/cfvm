use std::{collections::HashMap, convert::identity};

use crate::cfir::handles::{Symbol, SymbolRef};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Input {
    Reg(Reg),
    Label(Label),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Reg {
    VirtualReg(SymbolRef),
    PhysicalReg(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Label {
    BasicBlockLabel(Symbol),
    FunctionLabel(Symbol),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct MachineBasicBlock {
    pub label: Option<Symbol>,
    pub instrs: Vec<Instr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineFunction {
    pub name: Symbol,
    pub bbs: Vec<MachineBasicBlock>,
    pub symble_table: HashMap<Symbol, usize>,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Instr {
    // pub prefix: Vec<Symbol>,
    pub op: Symbol,
    pub input: Vec<Input>,
    pub output: Reg,
}

impl Instr {
    fn get_output(&self) -> Reg {
        self.output.clone()
    }
}

/// DAG Tree structs

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DAGTree {
    Op(Symbol, Vec<DAGTree>),
    Input(Input),
}

///

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpNodeList(pub Vec<Instr>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParellelTree (pub Vec<ParellelTree>, OpNodeList);

/// parellel tree:
/// - a
///   - b
///   - c
/// - d
///   - e
///   - f
/// parellel list:
/// [[b, c, e, f], [a, d]]
///
impl ParellelTree {
    fn expand(self) -> Vec<Vec<OpNodeList>> {
        let ParellelTree(parent, oplist) = self;
        parent.into_iter()
            .map(|x| x.expand())
            .reduce(parellel_list_merge)
            .map_or_else(|| vec![], identity)
    }
}

/// parellel_list_merge
/// merge [[b, c], [a]] [[e, f], [a]]
/// to [[b, c, e, f], [a, d]]
///
fn parellel_list_merge(mut x: Vec<Vec<OpNodeList>>, mut y: Vec<Vec<OpNodeList>>) -> Vec<Vec<OpNodeList>> {
    let max_len = x.len().max(y.len());
    x.extend(vec![vec![]; max_len-x.len()]);
    y.extend(vec![vec![]; max_len-y.len()]);
    x.into_iter().zip(y.into_iter())
        .map(|(mut x, y)| { x.extend(y); x })
        .collect()
}


// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct OpNode(pub Symbol, pub Vec<Input>);


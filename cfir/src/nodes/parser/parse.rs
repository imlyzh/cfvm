use pest_derive::*;

#[derive(Parser)]
#[grammar = "./nodes/parser/grammar.pest"]
pub enum CFIR {}

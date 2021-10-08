use pest::iterators::Pair;
use pest_derive::*;

use crate::nodes::*;
use crate::nodes::handles::*;


#[derive(Parser)]
#[grammar = "./nodes/parser/grammar.pest"]
pub enum CFIR {}


pub trait ParseFrom<T>
where
    Self: std::marker::Sized,
{
    fn parse_from(pair: Pair<T>) -> Self;
}


impl ParseFrom<Rule> for Module {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::module);
        let mut pairs = pair.into_inner();
        let name = Handle::new(pairs.next().unwrap().as_str().to_string());
        let bodys = pairs.next().unwrap().into_inner();

        let mut type_defs: HashMap<TypeDefineSymbol, TypeDef> = Default::default();
        let mut constant_defs: HashMap<DefineSymbol, ConstantDef> = Default::default();
        let mut variable_defs: HashMap<DefineSymbol, VariableDef> = Default::default();
        let mut functions: HashMap<DefineSymbol, FunctionDef> = Default::default();
        let mut function_decls: HashMap<DefineSymbol, FunctionDecl> = Default::default();

        for pair in bodys {
            match pair.as_rule() {
                Rule::type_def => {
                    let pair = pair.into_inner().next().unwrap();
                    let type_def = TypeDef::parse_from(pair);
                    let name = type_def.1.clone();
                    type_defs.insert(name, type_def);
                }
                Rule::constant_def => {
                    let pair = pair.into_inner().next().unwrap();
                    let constant_def = ConstantDef::parse_from(pair);
                    let name = constant_def.1.clone();
                    constant_defs.insert(name, constant_def);
                }
                Rule::variable_def => {
                    let pair = pair.into_inner().next().unwrap();
                    let variable_def = VariableDef::parse_from(pair);
                    let name = variable_def.1.clone();
                    variable_defs.insert(name, variable_def);
                }
                Rule::function_def => {
                    let pair = pair.into_inner().next().unwrap();
                    let function_def = FunctionDef::parse_from(pair);
                    let name = function_def.name.clone();
                    functions.insert(name, function_def);
                }
                Rule::function_decl => {
                    let pair = pair.into_inner().next().unwrap();
                    let function_decl = FunctionDecl::parse_from(pair);
                    let name = function_decl.name.clone();
                    function_decls.insert(name, function_decl);
                }
                _ => unreachable!(),
            }
        }
        Module {
            name,
            type_defs,
            constant_defs,
            variable_defs,
            functions,
            function_decls,
        }
    }
}

/// attr tags ////////////////////////////////////////////////////////////////////////////////////

impl ParseFrom<Rule> for IsPub {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::is_pub);
        if pair.as_str() == "pub" {
            IsPub(true)
        } else {
            IsPub(false)
        }
    }
}

impl ParseFrom<Rule> for IsNotAligned {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::is_not_aligned);
        if pair.as_str() == "#" {
            IsNotAligned(true)
        } else {
            IsNotAligned(false)
        }
    }
}

/// symbols ////////////////////////////////////////////////////////////////////////////////////

impl ParseFrom<Rule> for DefineSymbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::global_define_symbol);
        DefineSymbol(Handle::new(pair.as_str().to_string())) // fixme: register in global intern string pool
    }
}

impl ParseFrom<Rule> for TypeDefineSymbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_define_symbol);
        TypeDefineSymbol(Handle::new(pair.as_str().to_string())) // fixme: register in global intern string pool
    }
}

impl ParseFrom<Rule> for TypeSymbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_symbol);
        TypeSymbol(Handle::new(pair.as_str().to_string())) // fixme: register in global intern string pool
        // todo: pair namespace and name
    }
}

impl ParseFrom<Rule> for Symbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::symbol);
        Symbol(Handle::new(pair.as_str().to_string())) // fixme: register in global intern string pool
    }
}

fn optional_symbol_parse_from(pair: Pair<Rule>) -> Option<Symbol> {
    debug_assert_eq!(pair.as_rule(), Rule::params_name);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::symbol => Some(Symbol::parse_from(pair)), // fixme: register in global internstring pool
        Rule::UNDERLINE => None,
        _ => unreachable!(),
    }
}

/// types ////////////////////////////////////////////////////////////////////////////////////

impl ParseFrom<Rule> for SimpleType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::simple_type);
        todo!()
    }
}

impl ParseFrom<Rule> for ArrayType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::array_type);
        let mut pairs = pair.into_inner();
        let type_ = Type::parse_from(pairs.next().unwrap());
        let len = pairs.next().unwrap().as_str().parse::<u64>().unwrap();
        ArrayType(Box::new(type_), len)
    }
}

fn record_kv_pair(pair: Pair<Rule>) -> (Option<Symbol>, Type) {
    debug_assert_eq!(pair.as_rule(), Rule::record_kv_pair);
    let mut pairs = pair.into_inner();
    let key = optional_symbol_parse_from(pairs.next().unwrap());
    let value = Type::parse_from(pairs.next().unwrap());
    (key, value)
}

impl ParseFrom<Rule> for RecordType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::record_type);
        let mut pairs = pair.into_inner();
        let is_not_aligned = IsNotAligned::parse_from(pairs.next().unwrap());
        let kvs = pairs.map(record_kv_pair).collect();
        RecordType (is_not_aligned, kvs)
    }
}

impl ParseFrom<Rule> for FirstClassType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::first_class_type);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::opaque_type => FirstClassType::OpaqueType,
            Rule::simple_type => FirstClassType::SimpleType(SimpleType::parse_from(pair)),
            Rule::array_type => FirstClassType::Array(ArrayType::parse_from(pair)),
            Rule::record_type => FirstClassType::Record(RecordType::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for FunctionType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::function_type);
        todo!()
    }
}

impl ParseFrom<Rule> for Type {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::void_type => Type::Void,
            Rule::first_class_type => Type::FirstClassType(FirstClassType::parse_from(pair)),
            Rule::function_type => Type::FunctionType(FunctionType::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for TypeHandle {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_value);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::type_ => TypeHandle::new(Handle::new(Type::parse_from(pair))),
            Rule::type_symbol => TypeHandle::from(TypeSymbol::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

/// defs ////////////////////////////////////////////////////////////////////////////////////

impl ParseFrom<Rule> for TypeDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_def);
        let mut pairs = pair.into_inner();
        let is_pub = IsPub::parse_from(pairs.next().unwrap());
        let name = TypeDefineSymbol::parse_from(pairs.next().unwrap());
        let type_ = TypeHandle::parse_from(pairs.next().unwrap());
        TypeDef(is_pub, name, type_)
    }
}

impl ParseFrom<Rule> for ConstantDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::constant_def);
        let mut pairs = pair.into_inner();
        let is_pub = IsPub::parse_from(pairs.next().unwrap());
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        todo!()
    }
}


impl ParseFrom<Rule> for VariableDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::variable_def);
        let mut pairs = pair.into_inner();
        let is_pub = IsPub::parse_from(pairs.next().unwrap());
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        todo!()
    }
}

impl ParseFrom<Rule> for FunctionDecl {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::function_decl);
        let mut pairs = pair.into_inner();
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        todo!()
    }
}

impl ParseFrom<Rule> for FunctionDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::function_def);
        let mut pairs = pair.into_inner();
        todo!()
    }
}


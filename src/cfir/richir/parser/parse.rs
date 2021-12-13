use std::collections::BTreeSet;

use pest::iterators::Pair;
use pest_derive::*;

use crate::cfir::types::*;
use crate::cfir::handles::*;

#[derive(Parser)]
#[grammar = "./cfir/richir/parser/richir.pest"]
pub enum RICHIR {}

pub trait ParseFrom<T>
where
    Self: std::marker::Sized,
{
    fn parse_from(pair: Pair<T>) -> Self;
}

/* impl ParseFrom<Rule> for Module {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::module);
        let mut pairs = pair.into_inner();
        let name = Handle::new(pairs.next().unwrap().as_str().to_string());
        let bodys = pairs.next().unwrap().into_inner();

        let mut type_defs: HashMap<TypeDefineSymbol, Handle<TypeDef>> = Default::default();
        let mut constant_defs: HashMap<DefineSymbol, Handle<ConstantDef>> = Default::default();
        let mut variable_defs: HashMap<DefineSymbol, Handle<VariableDef>> = Default::default();
        let mut functions: HashMap<DefineSymbol, Handle<FunctionDef>> = Default::default();
        let mut function_decls: HashMap<DefineSymbol, Handle<FunctionDecl>> = Default::default();

        for pair in bodys {
            match pair.as_rule() {
                Rule::type_def => {
                    let pair = pair.into_inner().next().unwrap();
                    let type_def = Handle::new(TypeDef::parse_from(pair));
                    let name = type_def.1.clone();
                    type_defs.insert(name, type_def);
                }
                Rule::constant_def => {
                    let pair = pair.into_inner().next().unwrap();
                    let constant_def = Handle::new(ConstantDef::parse_from(pair));
                    let name = constant_def.1.clone();
                    constant_defs.insert(name, constant_def);
                }
                Rule::variable_def => {
                    let pair = pair.into_inner().next().unwrap();
                    let variable_def = Handle::new(VariableDef::parse_from(pair));
                    let name = variable_def.1.clone();
                    variable_defs.insert(name, variable_def);
                }
                Rule::function_def => {
                    let pair = pair.into_inner().next().unwrap();
                    let function_def = Handle::new(FunctionDef::parse_from(pair));
                    let name = function_def.name.clone();
                    functions.insert(name, function_def);
                }
                Rule::function_decl => {
                    let pair = pair.into_inner().next().unwrap();
                    let function_decl = Handle::new(FunctionDecl::parse_from(pair));
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
} */

/// attr tags /////////////////////////////////////////////////////////////////////////////////

/*
impl ParseFrom<Rule> for IsExtern {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::is_extern);
        if pair.as_str() == "extern" {
            IsExtern(true)
        } else {
            IsExtern(false)
        }
    }
}

impl ParseFrom<Rule> for IsPublic {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::is_pub);
        if pair.as_str() == "pub" {
            IsPublic(true)
        } else {
            IsPublic(false)
        }
    }
}

#[inline]
fn option_inline_type_parse_from(pair: Pair<Rule>) -> Option<InlineType> {
    debug_assert_eq!(pair.as_rule(), Rule::is_inline);
    if pair.as_str() == "inline" {
        Some(InlineType::Inline)
    } else if pair.as_str() == "const" {
        Some(InlineType::Const)
    } else {
        None
    }
}


impl ParseFrom<Rule> for IsExtend {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::is_extend);
        if pair.as_str() == "extend" {
            IsExtend(true)
        } else {
            IsExtend(false)
        }
    }
}

impl ParseFrom<Rule> for IsAtomic {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::is_atomic);
        if pair.as_str() == "atomic" {
            IsAtomic(true)
        } else {
            IsAtomic(false)
        }
    }
}

impl ParseFrom<Rule> for IsVolatile {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::is_volatile);
        if pair.as_str() == "volatile" {
            IsVolatile(true)
        } else {
            IsVolatile(false)
        }
    }
}
*/

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
        let pair = pair.into_inner().next().unwrap();
        DefineSymbol(Handle::new(pair.as_str().to_string())) // fixme: register in global intern string pool
    }
}

impl ParseFrom<Rule> for TypeSymbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_symbol);
        let mut pairs = pair.into_inner();
        let sym = TypeDefineSymbol::parse_from(pairs.next().unwrap());
        if let Some(x) = pairs.next() {
            let namespace = x.as_str().to_string();
            TypeSymbol(Some(Symbol(Handle::new(namespace))), sym)
        } else {
            TypeSymbol(None, sym)
        }
        // fixme: register in global intern string pool
    }
}

impl ParseFrom<Rule> for TypeDefineSymbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_define_symbol);
        TypeDefineSymbol(Handle::new(pair.as_str().to_string())) // fixme: register in global intern string pool
    }
}

impl ParseFrom<Rule> for LocalSymbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::local_symbol);
        let mut pairs = pair.into_inner();
        let sym = pairs.next().unwrap().as_str().to_string();
        LocalSymbol(Handle::new(sym))
    }
}

impl ParseFrom<Rule> for GlobalSymbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::global_symbol);
        let mut pairs = pair.into_inner();
        let sym = DefineSymbol::parse_from(pairs.next().unwrap());
        if let Some(x) = pairs.next() {
            let namespace = x.as_str().to_string();
            GlobalSymbol(Some(Symbol(Handle::new(namespace))), sym)
        } else {
            GlobalSymbol(None, sym)
        }
        // fixme: register in global intern string pool
    }
}

impl ParseFrom<Rule> for Symbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::symbol);
        Symbol(Handle::new(pair.as_str().to_string())) // fixme: register in global intern string pool
    }
}

#[inline]
fn optional_symbol_parse_from(pair: Pair<Rule>) -> Option<Symbol> {
    debug_assert_eq!(pair.as_rule(), Rule::params_name);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::symbol => Some(Symbol::parse_from(pair)), // fixme: register in global internstring pool
        Rule::UNDERLINE => None,
        _ => unreachable!(),
    }
}

#[inline]
fn optional_local_symbol_parse_from(pair: Pair<Rule>) -> Option<LocalSymbol> {
    debug_assert_eq!(pair.as_rule(), Rule::params_name);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::symbol => Some(LocalSymbol::parse_from(pair)), // fixme: register in global internstring pool
        Rule::UNDERLINE => None,
        _ => unreachable!(),
    }
}

/// types ////////////////////////////////////////////////////////////////////////////////////

impl ParseFrom<Rule> for IntType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::int_type);
        match pair.as_str() {
            "i1" => IntType::I1,
            "i8" => IntType::I8,
            "i16" => IntType::I16,
            "i32" => IntType::I32,
            "i64" => IntType::I64,
            "i128" => IntType::I128,
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for FloatType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::float_type);
        match pair.as_str() {
            "f8" => FloatType::F8,
            "f16" => FloatType::F16,
            "f32" => FloatType::F32,
            "f64" => FloatType::F64,
            "f128" => FloatType::F128,
            "ppc_f128" => FloatType::PpcF128,
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for PointerType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::pointer_type);
        let pair = pair.into_inner().next().unwrap();
        let type_ = Type::parse_from(pair);
        PointerType(Box::new(type_))
    }
}

impl ParseFrom<Rule> for VectorType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::vector_type);
        let mut pairs = pair.into_inner();
        let type_ = SimpleType::parse_from(pairs.next().unwrap());
        let len = pairs.next().unwrap().as_str().parse::<u64>().unwrap();
        VectorType(Box::new(type_), len)
    }
}

impl ParseFrom<Rule> for SimpleType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::simple_type);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::int_type => SimpleType::Int(IntType::parse_from(pair)),
            Rule::float_type => SimpleType::Float(FloatType::parse_from(pair)),
            Rule::pointer_type => SimpleType::Pointer(PointerType::parse_from(pair)),
            Rule::vector_type => SimpleType::Vector(VectorType::parse_from(pair)),
            _ => unreachable!(),
        }
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

#[inline]
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
        RecordType(is_not_aligned, kvs)
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

#[inline]
fn register_set_parse_from(pair: Pair<Rule>) -> BTreeSet<usize> {
    debug_assert_eq!(pair.as_rule(), Rule::reg_enum);
    pair.into_inner()
        .map(|p| p.as_str().parse::<usize>().unwrap())
        .collect()
}

#[inline]
fn register_range_parse_from(pair: Pair<Rule>) -> (usize, usize) {
    debug_assert_eq!(pair.as_rule(), Rule::reg_enum);
    let mut pairs = pair.into_inner();
    let left = pairs.next().unwrap().as_str().parse::<usize>().unwrap();
    let right = pairs.next().unwrap().as_str().parse::<usize>().unwrap();
    (left, right)
}

#[inline]
fn register_parse_from(pair: Pair<Rule>) -> usize {
    debug_assert_eq!(pair.as_rule(), Rule::reg_enum);
    pair.as_str().parse::<usize>().unwrap()
}

/*
impl ParseFrom<Rule> for RegisterType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::alloca_type_reg);
        let mut pairs = pair.into_inner();
        let is_extend = IsExtend::parse_from(pairs.next().unwrap());
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::reg_enum => RegisterType::Registers(is_extend, register_set_parse_from(pair)),
            Rule::reg_range => {
                RegisterType::RegisterRange(is_extend, register_range_parse_from(pair))
            }
            Rule::reg_number => RegisterType::Register(is_extend, register_parse_from(pair)),
            _ => unreachable!(),
        }
    }
}
*/

impl ParseFrom<Rule> for AllocaType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::alloca_type);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::alloca_type_stack => AllocaType::Stack,
            Rule::alloca_type_reg => todo!(),
            _ => unreachable!(),
        }
    }
}

#[inline]
fn option_alloca_type_parse_from(pair: Pair<Rule>) -> Option<AllocaType> {
    debug_assert_eq!(pair.as_rule(), Rule::alloca_type);
    if pair.as_str().is_empty() {
        None
    } else {
        Some(AllocaType::parse_from(pair))
    }
}

impl ParseFrom<Rule> for TypeBindAttr {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_bind_metadata);
        let mut pairs = pair.into_inner();
        let ty = Type::parse_from(pairs.next().unwrap());
        let alloc_type = pairs.next().map(AllocaType::parse_from);
        TypeBindAttr(Box::new(ty), alloc_type)
    }
}

#[inline]
fn params_pair_parse_from(pair: Pair<Rule>) -> (Option<LocalSymbol>, TypeBindAttr) {
    debug_assert_eq!(pair.as_rule(), Rule::params_pair);
    let mut pairs = pair.into_inner();
    let name = optional_local_symbol_parse_from(pairs.next().unwrap());
    let ty = TypeBindAttr::parse_from(pairs.next().unwrap());
    (name, ty)
}

impl ParseFrom<Rule> for ParamsType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::params);
        ParamsType(pair.into_inner().map(params_pair_parse_from).collect())
    }
}

impl ParseFrom<Rule> for FunctionType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::function_type);
        let mut pairs = pair.into_inner();
        let params = ParamsType::parse_from(pairs.next().unwrap());
        let return_type = TypeBindAttr::parse_from(pairs.next().unwrap());
        FunctionType {
            return_type,
            params,
        }
    }
}

impl ParseFrom<Rule> for Type {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::void_type => Type::Void,
            Rule::first_class_type => Type::FCType(FirstClassType::parse_from(pair)),
            Rule::function_type => Type::FunType(FunctionType::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

/// global values ///////////////////////////////////////////////////////////////////////////

impl ParseFrom<Rule> for VectorValue {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::vector_value);
        VectorValue(pair.into_inner().map(SimpleValue::parse_from).collect())
    }
}

impl ParseFrom<Rule> for ArrayValue {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::array_value);
        ArrayValue(pair.into_inner().map(ConstantValue::parse_from).collect())
    }
}

impl ParseFrom<Rule> for SimpleValue {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::simple_value);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::number => SimpleValue::Number(pair.as_str().to_string()),
            Rule::float_number => SimpleValue::FloatNumber(pair.as_str().to_string()),
            Rule::char => SimpleValue::Char('\0'), // todo: escape char
            Rule::vector_value => SimpleValue::Vector(VectorValue::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

#[inline]
fn record_value_kv_pair_parse_from(pair: Pair<Rule>) -> (Option<Symbol>, ConstantValue) {
    debug_assert_eq!(pair.as_rule(), Rule::record_value_kv_pair);
    let mut pairs = pair.into_inner();
    let name = optional_symbol_parse_from(pairs.next().unwrap());
    let value = ConstantValue::parse_from(pairs.next().unwrap());
    (name, value)
}

impl ParseFrom<Rule> for RecordValue {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::record_value);
        RecordValue(
            pair.into_inner()
                .map(record_value_kv_pair_parse_from)
                .collect(),
        )
    }
}

impl ParseFrom<Rule> for StringLit {
    fn parse_from(pair: Pair<Rule>) -> Self {
        StringLit(Handle::new(pair.as_str().to_string()))
        // todo string cast
    }
}

impl ParseFrom<Rule> for ConstantValue {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::constant_value);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::simple_value => ConstantValue::SimpleValue(SimpleValue::parse_from(pair)),
            Rule::array_value => ConstantValue::ArrayValue(ArrayValue::parse_from(pair)),
            Rule::record_value => ConstantValue::RecordValue(RecordValue::parse_from(pair)),
            Rule::string_lit => ConstantValue::StringLit(StringLit::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

/// defs ////////////////////////////////////////////////////////////////////////////////////

/*
impl ParseFrom<Rule> for TypeDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_def);
        let mut pairs = pair.into_inner();
        let is_pub = IsPublic::parse_from(pairs.next().unwrap());
        let name = TypeDefineSymbol::parse_from(pairs.next().unwrap());
        let type_ = TypeHandle::parse_from(pairs.next().unwrap());
        TypeDef(is_pub, name, type_)
    }
}

impl ParseFrom<Rule> for ConstantDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::constant_def);
        let mut pairs = pair.into_inner();
        let is_pub = IsPublic::parse_from(pairs.next().unwrap());
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        let ty = TypeHandle::parse_from(pairs.next().unwrap());
        let const_value = ConstantValue::parse_from(pairs.next().unwrap());
        ConstantDef(is_pub, name, ty, const_value)
    }
}

impl ParseFrom<Rule> for VariableDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::variable_def);
        let mut pairs = pair.into_inner();
        let is_pub = IsPublic::parse_from(pairs.next().unwrap());
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        let ty = TypeHandle::parse_from(pairs.next().unwrap());
        let const_value = pairs.next().map(ConstantValue::parse_from);
        VariableDef(is_pub, name, ty, const_value)
    }
}

impl ParseFrom<Rule> for FunctionDecl {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::function_decl);
        let mut pairs = pair.into_inner();
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        let header = FunctionType::parse_from(pairs.next().unwrap());
        FunctionDecl { name, header }
    }
}
*/

/// function def
fn foo() {

}

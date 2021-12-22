use std::collections::BTreeSet;
use std::collections::HashMap;

use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;
use pest_derive::*;

use crate::cfir::types::*;
use crate::cfir::handles::*;
use crate::cfir::base::*;
use crate::cfir::richir::*;

#[derive(Parser)]
#[grammar = "./cfir/richir/parser/richir.pest"]
pub enum RICHIR {}

pub trait ParseFrom<T>
where
    Self: std::marker::Sized,
{
    fn parse_from(pair: Pair<T>) -> Self;
}

/// attr tags /////////////////////////////////////////////////////////////////////////////////

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

impl ParseFrom<Rule> for SymbolRef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::symbol_ref);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::local_symbol => SymbolRef::Local(LocalSymbol::parse_from(pair)),
            Rule::global_symbol => SymbolRef::Global(GlobalSymbol::parse_from(pair)),
            Rule::symbol => SymbolRef::Symbol(Symbol::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

/// types ////////////////////////////////////////////////////////////////////////////////////

impl ParseFrom<Rule> for IntType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::int_type);
        let pair = pair.into_inner().next().unwrap();
        IntType(str::parse(pair.as_str()).unwrap())
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

impl ParseFrom<Rule> for RegType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::reg_type);
        match pair.as_str() {
            "int" => RegType::Int,
            "float" => RegType::Float,
            "simd" => RegType::Simd,
            "vector" => RegType::Vector,
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for RegPos {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::reg_pos);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::reg_enum => RegPos::Registers(register_set_parse_from(pair)),
            Rule::reg_range => {
                let (l, r) = register_range_parse_from(pair);
                RegPos::RegisterRange(l, r)
            },
            Rule::reg_number => RegPos::Register(register_parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for RegAllocaType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::alloca_type_reg);
        let mut pairs = pair.into_inner();
        let reg_type = RegType::parse_from(pairs.next().unwrap());
        let reg_pos = RegPos::parse_from(pairs.next().unwrap());
        RegAllocaType(reg_type, reg_pos)
    }
}

impl ParseFrom<Rule> for StoreType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::store_type);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_str() {
            "atomic" => StoreType::Atomic,
            "volatile" => StoreType::Volatile,
            _ => unreachable!()
        }
    }
}

#[inline]
fn store_type_opt_parse_from(pair: Pair<Rule>) -> Option<StoreType> {
    debug_assert_eq!(pair.as_rule(), Rule::store_type_opt);
    pair.into_inner().next().map(StoreType::parse_from)
}

impl ParseFrom<Rule> for AllocaType {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::alloca_type);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::alloca_type_stack => AllocaType::Stack(store_type_opt_parse_from(pair.into_inner().next().unwrap())),
            Rule::alloca_type_reg => AllocaType::Register(RegAllocaType::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

/*
#[inline]
fn option_alloca_type_parse_from(pair: Pair<Rule>) -> Option<AllocaType> {
    debug_assert_eq!(pair.as_rule(), Rule::alloca_type);
    if pair.as_str().is_empty() {
        None
    } else {
        Some(AllocaType::parse_from(pair))
    }
}
 */

impl ParseFrom<Rule> for TypeHandle {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_value);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::type_ => TypeHandle::Reference(Box::new(Type::parse_from(pair))),
            Rule::type_symbol => TypeHandle::Symbol(TypeSymbol::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for TypeBindAttr {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_bind_metadata);
        let mut pairs = pair.into_inner();
        let ty = TypeHandle::parse_from(pairs.next().unwrap());
        let alloc_type = pairs.next().map(AllocaType::parse_from);
        TypeBindAttr(ty, alloc_type)
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

impl ParseFrom<Rule> for Value {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::value);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::constant_value => Value::Lit(ConstantValue::parse_from(pair)),
            Rule::symbol_ref => Value::Var(SymbolRef::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

/// defs ////////////////////////////////////////////////////////////////////////////////////

impl ParseFrom<Rule> for TypeDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::type_def);
        let mut pairs = pair.into_inner();
        let is_pub = IsPublic::parse_from(pairs.next().unwrap());
        let name = TypeDefineSymbol::parse_from(pairs.next().unwrap());
        let type_ = Type::parse_from(pairs.next().unwrap());
        TypeDef {
            is_pub,
            name,
            type_,
        }
    }
}

impl ParseFrom<Rule> for ConstantDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::constant_def);
        let mut pairs = pair.into_inner();
        let is_pub = IsPublic::parse_from(pairs.next().unwrap());
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        let type_ = TypeSymbol::parse_from(pairs.next().unwrap());
        let value = ConstantValue::parse_from(pairs.next().unwrap());
        ConstantDef {
            is_pub,
            name,
            type_,
            value,
        }
    }
}

impl ParseFrom<Rule> for VariableDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::variable_def);
        let mut pairs = pair.into_inner();
        let is_pub = IsPublic::parse_from(pairs.next().unwrap());
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        let type_ = TypeSymbol::parse_from(pairs.next().unwrap());
        let value = pairs.next().map(ConstantValue::parse_from);
        VariableDef {
            is_pub,
            name,
            type_,
            value,
        }
    }
}

impl ParseFrom<Rule> for FunDecl {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::function_decl);
        let mut pairs = pair.into_inner();
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        let header = FunctionType::parse_from(pairs.next().unwrap());
        FunDecl {
            name,
            header,
        }
    }
}

impl ParseFrom<Rule> for FunctionAttr {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::function_attr);
        let mut pairs = pair.into_inner();
        let is_extern = IsExtern::parse_from(pairs.next().unwrap());
        let is_public = IsPublic::parse_from(pairs.next().unwrap());
        let is_inline = option_inline_type_parse_from(pairs.next().unwrap());
        FunctionAttr {
            is_extern,
            is_public,
            is_inline,
        }
    }
}

/// function def

impl ParseFrom<Rule> for NamedFun {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::function_def);
        let mut pairs = pair.into_inner();
        let attr = FunctionAttr::parse_from(pairs.next().unwrap());
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        let ftyp = FunctionType::parse_from(pairs.next().unwrap());
        let begin = Begin::parse_from(pairs.next().unwrap());
        let fun = Fun {
            ftyp,
            body: Box::new(Expr::Begin(begin)),
        };
        NamedFun {
            attr,
            name,
            fun,
        }
    }
}

impl ParseFrom<Rule> for Expr {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::expr);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::let_binding => Expr::Let(LetBinding::parse_from(pair)),
            Rule::conds => Expr::Cond(Cond::parse_from(pair)),
            Rule::if_expr => Expr::If(If::parse_from(pair)),
            Rule::switch => Expr::Switch(Switch::parse_from(pair)),
            Rule::while_expr => Expr::While(While::parse_from(pair)),
            Rule::begin => Expr::Begin(Begin::parse_from(pair)),
            Rule::store => Expr::Store(Store::parse_from(pair)),
            Rule::call => Expr::Val(Value::Call(Call::parse_from(pair))),
            Rule::value => Expr::Val(Value::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for LetBinding {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::let_binding);
        let mut pairs = pair.into_inner();
        let name = LocalSymbol::parse_from(pairs.next().unwrap());
        let type_ = type_bind_opt(pairs.next().unwrap());
        let value = call_or_value(pairs.next().unwrap());
        let expr = Expr::parse_from(pairs.next().unwrap());
        LetBinding {
            bind: (name, value, type_),
            body: Box::new(expr),
        }
    }
}

#[inline]
fn call_or_value(pair: Pair<Rule>) -> Value {
    debug_assert_eq!(pair.as_rule(), Rule::call_or_value);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::call => Value::Call(Call::parse_from(pair)),
        Rule::value => Value::parse_from(pair),
        _ => unreachable!(),
    }
}

#[inline]
fn type_bind_opt(pair: Pair<Rule>) -> Option<TypeBindAttr> {
    debug_assert_eq!(pair.as_rule(), Rule::type_bind_opt);
    pair.into_inner().next().map(TypeBindAttr::parse_from)
}

impl ParseFrom<Rule> for Cond {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::conds);
        let mut pairs = pair.into_inner();
        let cond_pairs = cond_pairs(pairs.next().unwrap());
        let els = Expr::parse_from(pairs.next().unwrap());
        Cond(cond_pairs, Box::new(els))
    }
}

impl ParseFrom<Rule> for If {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::conds);
        let mut pairs = pair.into_inner();
        let (value, then) = cond_pair(pairs.next().unwrap());
        let els = Expr::parse_from(pairs.next().unwrap());
        If(value, Box::new(then), Box::new(els))
    }
}

#[inline]
fn cond_pairs(pair: Pair<Rule>) -> Vec<(Value, Expr)> {
    debug_assert_eq!(pair.as_rule(), Rule::cond_pairs);
    pair.into_inner().map(cond_pair).collect()
}

#[inline]
fn cond_pair(pair: Pair<Rule>) -> (Value, Expr) {
    debug_assert_eq!(pair.as_rule(), Rule::cond_pair);
    let mut pairs = pair.into_inner();
    let cond = Value::parse_from(pairs.next().unwrap());
    let body = Expr::parse_from(pairs.next().unwrap());
    (cond, body)
}

impl ParseFrom<Rule> for Switch {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::switch);
        let mut pairs = pair.into_inner();
        let cond = Value::parse_from(pairs.next().unwrap());
        let cases = switch_cases(pairs.next().unwrap());
        let default = default_case(pairs.next().unwrap());
        Switch(cond, cases, Box::new(default))
    }
}

#[inline]
fn switch_cases(pair: Pair<Rule>) -> Vec<(ConstantValue, Expr)> {
    debug_assert_eq!(pair.as_rule(), Rule::switch_cases);
    pair.into_inner().map(switch_case).collect()
}

#[inline]
fn switch_case(pair: Pair<Rule>) -> (ConstantValue, Expr) {
    debug_assert_eq!(pair.as_rule(), Rule::switch_case);
    let mut pairs = pair.into_inner();
    let value = ConstantValue::parse_from(pairs.next().unwrap());
    let body = Expr::parse_from(pairs.next().unwrap());
    (value, body)
}

#[inline]
fn default_case(pair: Pair<Rule>) -> Expr {
    debug_assert_eq!(pair.as_rule(), Rule::default_case);
    Expr::parse_from(pair.into_inner().next().unwrap())
}

impl ParseFrom<Rule> for Begin {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::begin);
        let pairs = pair.into_inner();
        let exprs = pairs.map(Expr::parse_from).collect();
        Begin(exprs)
    }
}

impl ParseFrom<Rule> for While {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::while_expr);
        let mut pairs = pair.into_inner();
        let cond = Value::parse_from(pairs.next().unwrap());
        let body = Expr::parse_from(pairs.next().unwrap());
        let accum = pairs.next().map(Store::parse_from);
        While(cond, Box::new(body), accum)
    }
}

impl ParseFrom<Rule> for Store {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::store);
        let mut pairs = pair.into_inner();
        let name = SymbolRef::parse_from(pairs.next().unwrap());
        let store_type = StoreType::parse_from(pairs.next().unwrap());
        let value = Expr::parse_from(pairs.next().unwrap());
        Store(name, store_type, Box::new(value))
    }
}

impl ParseFrom<Rule> for Call {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::call);
        let mut pairs = pair.into_inner();
        let fun = Value::parse_from(pairs.next().unwrap());
        let args: Vec<Value> = pairs.map(Value::parse_from).collect();
        Call {
            fun: Box::new(fun),
            args,
        }
    }
}

// module pars

impl ParseFrom<Rule> for Module<NamedFun> {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::module);
        let mut pairs = pair.into_inner();
        let name = Symbol::parse_from(pairs.next().unwrap());
        let bodys = pairs.next().unwrap().into_inner();

        let mut type_defs: HashMap<TypeDefineSymbol, TypeDef> = Default::default();
        let mut constant_defs: HashMap<DefineSymbol, ConstantDef> = Default::default();
        let mut variable_defs: HashMap<DefineSymbol, VariableDef> = Default::default();
        let mut function_defs: HashMap<DefineSymbol, NamedFun> = Default::default();
        let mut function_decls: HashMap<DefineSymbol, FunDecl> = Default::default();

        for pair in bodys {
            match dbg!(pair.as_rule()) {
                Rule::type_def => {
                    let type_def = TypeDef::parse_from(pair);
                    let name = type_def.name.clone();
                    type_defs.insert(name, type_def);
                }
                Rule::constant_def => {
                    let constant_def = ConstantDef::parse_from(pair);
                    let name = constant_def.name.clone();
                    constant_defs.insert(name, constant_def);
                }
                Rule::variable_def => {
                    let variable_def = VariableDef::parse_from(pair);
                    let name = variable_def.name.clone();
                    variable_defs.insert(name, variable_def);
                }
                Rule::function_def => {
                    let function_def = NamedFun::parse_from(pair);
                    let name = function_def.name.clone();
                    function_defs.insert(name, function_def);
                }
                Rule::function_decl => {
                    let function_decl = FunDecl::parse_from(pair);
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
            function_defs,
            function_decls,
        }
    }
}

pub fn file_parse(input: &str) -> Result<Vec<Module<NamedFun>>, Error<Rule>> {
    let mut p = dbg!(RICHIR::parse(Rule::file, input))?;
    let r = p.next().unwrap()
        .into_inner()
        .filter(|x| x.as_rule() == Rule::module)
        .map(|x| Module::parse_from(x))
        .collect();
    Ok(r)
}
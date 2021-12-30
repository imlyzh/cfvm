use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::HashMap;

use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest_derive::*;

use crate::cfir::types::*;
use crate::cfir::handles::*;
use crate::cfir::base::*;
use crate::cfir::graphir::*;
use crate::cfir::graphir::instruction::*;

#[derive(Parser)]
#[grammar = "./cfir/graphir/parser/graphir.pest"]
pub enum GRAPHIR {}

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
        DefineSymbol(Symbol::parse_from(pair)) // fixme: register in global intern string pool
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
        // let pair = pair.into_inner().next().unwrap();
        TypeDefineSymbol(Handle::new(pair.as_str().to_string())) // fixme: register in global intern string pool // fixme: register in global intern string pool
    }
}

impl ParseFrom<Rule> for LocalSymbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::local_symbol);
        let pair = pair.into_inner().next().unwrap();
        LocalSymbol(Symbol::parse_from(pair))
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

impl ParseFrom<Rule> for LabelSymbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::label_symbol);
        let pair = pair.into_inner().next().unwrap();
        LabelSymbol(Symbol::parse_from(pair))
    }
}

/// function def

#[inline]
fn insts_parse_from(pair: Pair<Rule>) -> Vec<LTMHand<Instruction>> {
    debug_assert_eq!(pair.as_rule(), Rule::insts);
    pair
        .into_inner()
        .map(|x| LTMHand::new(Instruction::parse_from(x)))
        .collect()
}

/*
#[inline]
pub fn label_symbol_opt_parse_from(pair: Pair<Rule>) -> Option<LabelSymbol> {
    debug_assert_eq!(pair.as_rule(), Rule::label_symbol_opt);
    pair.into_inner().next().map(LabelSymbol::parse_from)
}
 */

impl ParseFrom<Rule> for BasicBlockDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        let t = pair.as_rule();
        debug_assert!(matches!(t, Rule::basic_block | Rule::begin_basic_block));
        let mut pairs = pair.into_inner();
        let label = if let Rule::basic_block = t {
            LabelSymbol::parse_from(pairs.next().unwrap())
        } else {
            todo!()
        };
        let instructions = LTMHand::new(insts_parse_from(pairs.next().unwrap()));
        let terminator = pairs.next().map(|x| LTMHand::new(Terminator::parse_from(x)));
        BasicBlockDef {
            label,
            instructions,
            terminator,
        }
    }
}

#[inline]
fn blocks_parse_from(pair: Pairs<Rule>) -> Vec<LTMHand<BasicBlockDef>> {
    pair.map(|x| LTMHand::new(BasicBlockDef::parse_from(x)))
        .collect()
}

impl ParseFrom<Rule> for FunctionDef {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::function_def);
        let mut pairs = pair.into_inner();
        let attr = FunctionAttr::parse_from(pairs.next().unwrap());
        let name = DefineSymbol::parse_from(pairs.next().unwrap());
        let header = FunctionType::parse_from(pairs.next().unwrap());
        let blocks = blocks_parse_from(pairs);
        // let block_map = HashMap::new();
        /*
        let block_map = blocks
            .iter()
            .enumerate()
            .map(|(usize, x)| (x.read().unwrap(), usize))
            .collect();
        let block_map = Handle::new(RwLock::new(block_map));
         */
        FunctionDef {
            name,
            header,
            function_attr: attr,
            bbs: RefCell::new(blocks),
            // block_map,
        }
    }
}

// terminators /////////////////////////////////////////////////////////////////////////////////

impl ParseFrom<Rule> for Ret {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::ret);
        let value = pair.into_inner().next().map(SymbolRef::parse_from);
        Ret(value)
    }
}

impl ParseFrom<Rule> for Branch {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::branch);
        let mut pairs = pair.into_inner();
        let branch_op = BranchOp::parse_from(pairs.next().unwrap());
        let cond = SymbolRef::parse_from(pairs.next().unwrap());
        let true_block = LabelSymbol::parse_from(pairs.next().unwrap());
        let false_block = LabelSymbol::parse_from(pairs.next().unwrap());
        Branch(
            branch_op,
            cond,
            true_block,
            false_block,
        )
    }
}

/*
fn conds_pair_parse_from(pair: Pair<Rule>) -> {
}
impl ParseFrom<Rule> for Conds {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::conds);
    }
}
*/

#[inline]
fn switch_pair_parse_from(pair: Pair<Rule>) -> (SimpleValue, LabelSymbol) {
    debug_assert_eq!(pair.as_rule(), Rule::switch_pair);
    let mut pairs = pair.into_inner();
    let value = SimpleValue::parse_from(pairs.next().unwrap());
    let label = LabelSymbol::parse_from(pairs.next().unwrap());
    (value, label)
}

impl ParseFrom<Rule> for Switch {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::switch);
        let mut pairs = pair.into_inner();
        let cond = SymbolRef::parse_from(pairs.next().unwrap());
        let map = pairs.map(switch_pair_parse_from).collect();
        Switch(cond, map)
    }
}

impl ParseFrom<Rule> for Terminator {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::terminator);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::ret => Terminator::Ret(Ret::parse_from(pair)),
            Rule::branch => Terminator::Branch(Branch::parse_from(pair)),
            // Rule::conds => Terminator::Conds(Conds::parse_from(pair)),
            Rule::switch => Terminator::Switch(Switch::parse_from(pair)),
            Rule::unrechable => Terminator::Unrechable,
            _ => unreachable!(),
        }
    }
}

/// insts //////////////////////////////////////////////////////////////////////////////////////

impl ParseFrom<Rule> for Store {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::store);
        let mut pairs = pair.into_inner();
        let dst = LocalSymbol::parse_from(pairs.next().unwrap());
        let ty = TypeBindAttr::parse_from(pairs.next().unwrap());
        let src = SymbolRef::parse_from(pairs.next().unwrap());
        Store(dst, src, ty)
    }
}

impl ParseFrom<Rule> for Index {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::index);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::symbol => Index::Symbol(Symbol::parse_from(pair)),
            Rule::number => Index::Index(pair.as_str().parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for IndexList {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::index_list);
        IndexList(pair.into_inner().map(Index::parse_from).collect())
    }
}

impl ParseFrom<Rule> for BindOperator {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::bind);
        let mut pairs = pair.into_inner();
        let symbol = LocalSymbol::parse_from(pairs.next().unwrap());
        let operator = Operator::parse_from(pairs.next().unwrap());
        BindOperator(
            symbol,
            LTMHand::new(operator),
            // ty,
        )
    }
}

impl ParseFrom<Rule> for Instruction {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::inst);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::store => Instruction::Store(Store::parse_from(pair)),
            Rule::bind => Instruction::BindOperator(BindOperator::parse_from(pair)),
            Rule::operator => {
                Instruction::Operator(LTMHand::new(Operator::parse_from(pair)))
            }
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for ICmpOp {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::icmp_op);
        match pair.as_str() {
            "eq" => ICmpOp::Eq,
            "ne" => ICmpOp::Ne,
            "sge" => ICmpOp::Sge,
            "sgt" => ICmpOp::Sgt,
            "sle" => ICmpOp::Sle,
            "slt" => ICmpOp::Slt,
            "uge" => ICmpOp::Uge,
            "ugt" => ICmpOp::Ugt,
            "ule" => ICmpOp::Ule,
            "ult" => ICmpOp::Ult,
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for FCmpOp {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::icmp_op);
        match pair.as_str() {
            "false" => FCmpOp::False,
            "oeq" => FCmpOp::Oeq,
            "oge" => FCmpOp::Oge,
            "ogt" => FCmpOp::Ogt,
            "ole" => FCmpOp::Ole,
            "olt" => FCmpOp::Olt,
            "one" => FCmpOp::One,
            "ord" => FCmpOp::Ord,
            "true" => FCmpOp::True,
            "ueq" => FCmpOp::Ueq,
            "uge" => FCmpOp::Uge,
            "ugt" => FCmpOp::Ugt,
            "ule" => FCmpOp::Ule,
            "ult" => FCmpOp::Ult,
            "une" => FCmpOp::Une,
            "uno" => FCmpOp::Uno,
            _ => unreachable!(),
        }
    }
}

#[inline]
fn phi_pair_parse_from(pair: Pair<Rule>) -> (LabelSymbol, SymbolRef) {
    debug_assert_eq!(pair.as_rule(), Rule::phi_pair);
    let mut pairs = pair.into_inner();
    let label = LabelSymbol::parse_from(pairs.next().unwrap());
    let value = SymbolRef::parse_from(pairs.next().unwrap());
    (label, value)
}

#[inline]
fn alloca_type_opt_parse_from(pair: Pair<Rule>) -> Option<AllocaType> {
    debug_assert_eq!(pair.as_rule(), Rule::alloca_type_opt);
    pair.into_inner().next().map(AllocaType::parse_from)
}

impl ParseFrom<Rule> for Operator {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::operator);
        let pair = pair.into_inner().next().unwrap();
        let t = pair.as_rule();
        let mut pairs = pair.into_inner();
        match t {
            Rule::alloca => {
                // let ty = TypeSymbol::parse_from(pairs.next().unwrap());
                // let aty = alloca_type_opt_parse_from(pairs.next().unwrap());
                let ty = TypeBindAttr::parse_from(pairs.next().unwrap());
                let value = pairs.next().map(SymbolRef::parse_from);
                // Operator::Alloca(ty, aty, value)
                Operator::Alloca(ty, value)
            }
            Rule::get_ptr => {
                let value = SymbolRef::parse_from(pairs.next().unwrap());
                let index = pairs.next().map(IndexList::parse_from);
                Operator::GetPtr(value, index)
            }
            Rule::load => {
                let ty = Type::parse_from(pairs.next().unwrap());
                let value = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::Load(ty, value)
            }
            Rule::cast => {
                let ty = Type::parse_from(pairs.next().unwrap());
                let value = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::Cast(ty, value)
            }
            Rule::add => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::Add(value1, value2)
            }
            Rule::fadd => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::FAdd(value1, value2)
            }
            Rule::sub => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::Sub(value1, value2)
            }
            Rule::fsub => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::FSub(value1, value2)
            }
            Rule::mul => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::Mul(value1, value2)
            }
            Rule::fmul => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::FMul(value1, value2)
            }
            Rule::udiv => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::UDiv(value1, value2)
            }
            Rule::sdiv => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::SDiv(value1, value2)
            }
            Rule::urem => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::URem(value1, value2)
            }
            Rule::srem => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::SRem(value1, value2)
            }
            Rule::frem => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::FRem(value1, value2)
            }
            Rule::shl => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::Shl(value1, value2)
            }
            Rule::lshr => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::LShr(value1, value2)
            }
            Rule::ashr => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::AShr(value1, value2)
            }
            Rule::and => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::And(value1, value2)
            }
            Rule::or => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::Or(value1, value2)
            }
            Rule::xor => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::Xor(value1, value2)
            }
            Rule::get_value => {
                let value = SymbolRef::parse_from(pairs.next().unwrap());
                let index = IndexList::parse_from(pairs.next().unwrap());
                Operator::GetValue(value, index)
            }
            Rule::get_item => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let index = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::GetItem(value1, index)
            }
            Rule::set_value => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let index = IndexList::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::SetValue(value1, index, value2)
            }
            Rule::set_item => {
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let index = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::SetItem(value1, index, value2)
            }
            Rule::trunc => {
                let ty = IntType::parse_from(pairs.next().unwrap());
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::Trunc(value1, ty)
            }
            Rule::zext => {
                let ty = IntType::parse_from(pairs.next().unwrap());
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::ZExt(value1, ty)
            }
            Rule::sext => {
                let ty = IntType::parse_from(pairs.next().unwrap());
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::SExt(value1, ty)
            }
            Rule::ftrunc => {
                let ty = FloatType::parse_from(pairs.next().unwrap());
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::FTrunc(value1, ty)
            }
            Rule::fext => {
                let ty = FloatType::parse_from(pairs.next().unwrap());
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::FExt(value1, ty)
            }
            Rule::icmp => {
                let op = ICmpOp::parse_from(pairs.next().unwrap());
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::ICmp(op, value1, value2)
            }
            Rule::fcmp => {
                let op = FCmpOp::parse_from(pairs.next().unwrap());
                let value1 = SymbolRef::parse_from(pairs.next().unwrap());
                let value2 = SymbolRef::parse_from(pairs.next().unwrap());
                Operator::FCmp(op, value1, value2)
            }
            Rule::phi => {
                let pairs = pairs.next().unwrap().into_inner();
                Operator::Phi(pairs.map(phi_pair_parse_from).collect())
            }
            Rule::call => {
                let callee = SymbolRef::parse_from(pairs.next().unwrap());
                let args = pairs
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(SymbolRef::parse_from)
                    .collect();
                Operator::Call(callee, args)
            }
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for BranchOp {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::branch_op);
        if pair.as_str() == "if-nil" {
            BranchOp::IfNil
        } else if pair.as_str() == "if-non-nil" {
            BranchOp::IfNonNil
        } else {
            BranchOp::IfNil
        }
    }
}

// module pars

impl ParseFrom<Rule> for Module<FunctionDef> {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::module);
        let mut pairs = pair.into_inner();
        let name = Symbol::parse_from(pairs.next().unwrap());
        let bodys = pairs.next().unwrap().into_inner();

        let mut type_defs: HashMap<TypeDefineSymbol, TypeDef> = Default::default();
        let mut constant_defs: HashMap<DefineSymbol, ConstantDef> = Default::default();
        let mut variable_defs: HashMap<DefineSymbol, VariableDef> = Default::default();
        let mut function_defs: HashMap<DefineSymbol, FunctionDef> = Default::default();
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
                    let function_def = FunctionDef::parse_from(pair);
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

pub fn file_parse(input: &str) -> Result<Vec<Module<FunctionDef>>, Error<Rule>> {
    let mut p = dbg!(GRAPHIR::parse(Rule::file, input))?;
    let r = p.next().unwrap()
        .into_inner()
        .filter(|x| x.as_rule() == Rule::module)
        .map(Module::parse_from)
        .collect();
    Ok(r)
}
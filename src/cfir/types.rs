use std::collections::BTreeSet;

use super::handles::{Symbol, LocalSymbol, TypeHandle};

pub trait GetType {
    fn get_type(&self) -> Type;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Void,
    FCType(FirstClassType),
    FunType(FunctionType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FirstClassType {
    OpaqueType,
    SimpleType(SimpleType),
    Array(ArrayType),
    Record(RecordType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SimpleType {
    Int(IntType),
    Float(FloatType),
    Pointer(PointerType),
    Reference(PointerType),
    Vector(VectorType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PointerType(pub Box<Type>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VectorType(pub Box<SimpleType>, pub u64);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArrayType(pub Box<Type>, pub u64);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IsNotAligned(pub bool);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RecordType(pub IsNotAligned, pub Vec<(Option<Symbol>, Type)>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeBindAttr(pub TypeHandle, pub Option<AllocaType>);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RegType {
    Int,
    Float,
    Simd,
    Vector,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RegPos {
    Register(usize),
    RegisterRange(usize, usize),
    Registers(BTreeSet<usize>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegAllocaType (pub RegType, pub RegPos);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StoreType {
    Volatile,
    Atomic,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AllocaType {
    Register(RegAllocaType),
    Stack(Option<StoreType>),
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParamsType(pub Vec<(Option<LocalSymbol>, TypeBindAttr)>);

impl ParamsType {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionType {
    pub return_type: TypeBindAttr,
    pub params: ParamsType,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
// #[repr(u8)]
pub struct IntType(pub u64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum FloatType {
    F8 = 0,
    F16 = 1,
    F32 = 2,
    F64 = 3,
    F128 = 4,
    PpcF128 = 5,
}

/*
pub trait Unify {
    type Target;
    fn unify(&self, other: &Self) -> Option<Self::Target>;
}

impl Unify for Type {
    type Target = Self;

    fn unify(&self, other: &Self) -> Option<Self::Target> {
        match (self, other) {
            (Type::Void, _) => Some(Type::Void),
            (t, Type::Void) => Some(t.clone()),
            (Type::FCType(t1), Type::FCType(t2)) => {
                t1.unify(t2).map(Type::FCType)
            }
            (Type::FunType(t1), Type::FunType(t2)) => {
                t1.unify(t2).map(Type::FunType)
            }
            _ => None,
        }
    }
}

impl Unify for FirstClassType {
    type Target = Self;

    fn unify(&self, other: &Self) -> Option<Self::Target> {
        match (self, other) {
            (FirstClassType::OpaqueType, _) => Some(FirstClassType::OpaqueType),
            (t, FirstClassType::OpaqueType) => Some(t.clone()),
            (FirstClassType::SimpleType(t1), FirstClassType::SimpleType(t2)) => todo!(),
            (FirstClassType::Array(t1), FirstClassType::Array(t2)) => todo!(),
            (FirstClassType::Record(t1), FirstClassType::Record(t2)) => todo!(),
            _ => None,
        }
    }
}

impl Unify for FunctionType {
    type Target = Self;

    fn unify(&self, other: &Self) -> Option<Self::Target> {
        let return_type = if self.return_type.0 == other.return_type.0 {
            TypeBindAttr(
                Box::new(self.return_type.0.unify(&other.return_type.0)?),
                self.return_type.1.clone(),
            )
        } else {
            return None;
        };
        let params: Option<Vec<(_, _)>> = self
            .params
            .0
            .iter()
            .zip(other.params.0.iter())
            .map(|((_, TypeBindAttr(t1, a1)), (_, TypeBindAttr(t2, a2)))| {
                if a1 == a2 {
                    Some((a1.clone(), t1.unify(t2)?))
                } else {
                    None
                }
            })
            .collect();
        let params = params?;
        let params = params
            .into_iter()
            .map(|(a, t)| (None, TypeBindAttr(Box::new(t), a)))
            .collect();
        let params = ParamsType(params);
        let r = FunctionType {
            return_type,
            params,
        };
        Some(r)
    }
}
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlarformWidth {
    pub data_size: u64,
    pub ptr_size: u64,
    pub aligned_size: u64,
}

pub trait GetSize {
    fn get_size(&self, platform_size: PlarformWidth) -> Option<u64>;
}

impl GetSize for Type {
    fn get_size(&self, platform_size: PlarformWidth) -> Option<u64> {
        if let Type::FCType(t) = self {
            t.get_size(platform_size)
        } else {
            None
        }
    }
}

impl GetSize for FirstClassType {
    fn get_size(&self, platform_size: PlarformWidth) -> Option<u64> {
        match self {
            FirstClassType::OpaqueType => None,
            FirstClassType::SimpleType(t) => t.get_size(platform_size),
            FirstClassType::Array(t) => t.get_size(platform_size),
            FirstClassType::Record(t) => t.get_size(platform_size),
        }
    }
}

impl GetSize for SimpleType {
    fn get_size(&self, platform_size: PlarformWidth) -> Option<u64> {
        match self {
            SimpleType::Int(t) => t.get_size(platform_size),
            SimpleType::Float(t) => t.get_size(platform_size),
            SimpleType::Pointer(_) | SimpleType::Reference(_) => Some(size_align(
                platform_size.ptr_size,
                platform_size.aligned_size
            ) as u64),
            SimpleType::Vector(t) => t.get_size(platform_size),
        }
    }
}

impl GetSize for VectorType {
    fn get_size(&self, platform_size: PlarformWidth) -> Option<u64> {
        let VectorType(t, s) = self;
        t.get_size(platform_size).map(|x| size_align(x * s, platform_size.aligned_size))
    }
}

impl GetSize for ArrayType {
    fn get_size(&self, platform_size: PlarformWidth) -> Option<u64> {
        let ArrayType(t, s) = self;
        t.get_size(platform_size).map(|x| size_align(x * s, platform_size.aligned_size))
    }
}

fn size_align(i: u64, aligned_size: u64) -> u64 {
    let platform_size = aligned_size as u64;
    let i_mod = i % platform_size;
    if i_mod == 0 {
        i
    } else {
        i + platform_size - i_mod
    }
}

impl GetSize for RecordType {
    fn get_size(&self, platform_size: PlarformWidth) -> Option<u64> {
        let r = self.1.iter().map(|(_, t)| t.get_size(platform_size));
        if self.0 .0 {
            r.sum()
        } else {
            r.map(|x| x.map(|x| size_align(x, platform_size.aligned_size))).sum()
        }
    }
}

impl GetSize for IntType {
    fn get_size(&self, platform_size: PlarformWidth) -> Option<u64> {
        Some(size_align(self.0, platform_size.aligned_size))
    }
}

impl GetSize for FloatType {
    fn get_size(&self, platform_size: PlarformWidth) -> Option<u64> {
        let map_array = [8, 16, 31, 64, 128, 128];
        Some(size_align(map_array[*self as usize], platform_size.aligned_size))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsExtern(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsPublic(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsAtomic(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineType {
    Inline,
    Const,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAttr {
    pub is_extern: IsExtern,
    // pub is_public: IsPublic,
    pub is_inline: Option<InlineType>,
}

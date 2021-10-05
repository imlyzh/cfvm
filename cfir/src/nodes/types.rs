use std::sync::Arc;

use super::{handles::LocalSymbol, instruction::AllocaType};


pub trait GetType {
    fn get_type(&self) -> Type;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Void,
    FirstClassType(FirstClassType),
    FunctionType(FunctionType),
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
    Vector(VectorType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PointerType(pub Box<Type>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VectorType(pub Box<SimpleType>, pub u64);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArrayType(pub Box<Type>, pub u64);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RecordType {
    pub not_aligned: bool,
    pub record: Vec<(Option<Arc<String>>, Type)>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionType {
    pub return_type: Box<Type>,
    pub params: Vec<(Option<LocalSymbol>,
        Option<AllocaType>, Type)>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum IntType {
    I1 = 0,
    I8 = 1,
    I16 = 2,
    I32 = 3,
    I64 = 4,
    I128 = 5,
}

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
            (Type::FirstClassType(t1), Type::FirstClassType(t2)) =>
                t1.unify(t2).map(Type::FirstClassType),
            (Type::FunctionType(t1), Type::FunctionType(t2)) =>
                t1.unify(t2).map(Type::FunctionType),
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
        let return_type = self.return_type.unify(&other.return_type)?;
        let return_type = Box::new(return_type);
        let params: Option<Vec<(_, _)>> = self.params
            .iter()
            .zip(other.params.iter())
            .map(|((_, a1, t1), (_, a2, t2))|
                if a1 == a2 {
                    Some((a1.clone(), t1.unify(t2)?))
                } else {
                    None
                })
            .collect();
        let params = params?;
        let params = params.into_iter().map(|(a, t)| (None, a, t)).collect();
        let r = FunctionType { return_type, params };
        Some(r)
    }
}


pub trait GetSize {
    fn get_size(&self, platform_size: u8) -> Option<u64>;
}

impl GetSize for Type {
    fn get_size(&self, platform_size: u8) -> Option<u64> {
        if let Type::FirstClassType(t) = self {
            t.get_size(platform_size)
        } else {
            None
        }
    }
}

impl GetSize for FirstClassType {
    fn get_size(&self, platform_size: u8) -> Option<u64> {
        match self {
            FirstClassType::OpaqueType => None,
            FirstClassType::SimpleType(t) => t.get_size(platform_size),
            FirstClassType::Array(t) => t.get_size(platform_size),
            FirstClassType::Record(t) => t.get_size(platform_size),
        }
    }
}

impl GetSize for SimpleType {
    fn get_size(&self, platform_size: u8) -> Option<u64> {
        match self {
            SimpleType::Int(t) => t.get_size(platform_size),
            SimpleType::Float(t) => t.get_size(platform_size),
            SimpleType::Pointer(_) => Some(platform_size as u64),
            SimpleType::Vector(t) => t.get_size(platform_size),
        }
    }
}

impl GetSize for VectorType {
    fn get_size(&self, platform_size: u8) -> Option<u64> {
        let VectorType(t, s) = self;
        t.get_size(platform_size).map(|x|x*s)
    }
}

impl GetSize for ArrayType {
    fn get_size(&self, platform_size: u8) -> Option<u64> {
        let ArrayType(t, s) = self;
        t.get_size(platform_size).map(|x|x*s)
    }
}

fn size_align(i: u64, platform_size: u8) -> u64 {
    let platform_size  = platform_size as u64;
    let i_mod = i % platform_size;
    if i_mod == 0 {
        i
    } else {
        i + platform_size - i_mod
    }
}

impl GetSize for RecordType {
    fn get_size(&self, platform_size: u8) -> Option<u64> {
        let r = self.record.iter().map(|(_, t)| t.get_size(platform_size));
        if self.not_aligned {
            r.sum()
        } else {
            r.map(|x| x.map(|x| size_align(x,  platform_size))).sum()
        }
    }
}

impl GetSize for IntType {
    fn get_size(&self, _: u8) -> Option<u64> {
        let map_array = [1, 1, 2, 4, 8, 16];
        Some(map_array[*self as usize])
    }
}

impl GetSize for FloatType {
    fn get_size(&self, _: u8) -> Option<u64> {
        let map_array = [1, 2, 4, 8, 16, 16];
        Some(map_array[*self as usize])
    }
}

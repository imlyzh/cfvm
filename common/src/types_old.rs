use std::ptr::NonNull;

pub trait GetType {
  fn get_type(&self) -> Type;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
  Void,
  OpaqueType,
  SimpleType(SimpleType),
  Array(ArrayType),
  Record(RecordType),
  FunType(FunctionType),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleType {
  Int(IntType),
  Float(FloatType),
  Pointer(PointerType),
  Reference(PointerType),
  Vector(VectorType),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointerType(pub Box<Type>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VectorType(pub Box<SimpleType>, pub u64);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrayType(pub Box<Type>, pub u64);

pub type IsNotAligned = bool;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecordType(pub IsNotAligned, pub Vec<(Option<NonNull<str>>, Type)>);

/*
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RegType {
  Int,
  Float,
  Simd,
  Vector,
}
// */

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoreType {
  Volatile,
  Atomic,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllocaType {
  Register,
  // Register(RegType),
  Stack(Option<StoreType>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParamsType(pub Vec<SimpleType>);

impl ParamsType {
  pub fn len(&self) -> usize {
    self.0.len()
  }
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionType {
  pub return_type: SimpleType,
  pub params: ParamsType,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
// #[repr(u8)]
pub struct IntType(pub u64);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum FloatType {
  // F8      = 0,
  // F16     = 1,
  F32 = 2,
  F64 = 3,
  // F128    = 4,
  // PpcF128 = 5,
}

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
    match self {
      Type::Void => Some(0),
      Type::OpaqueType => None,
      Type::SimpleType(t) => t.get_size(platform_size),
      Type::Array(t) => t.get_size(platform_size),
      Type::Record(t) => t.get_size(platform_size),
      Type::FunType(_) => None,
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
        platform_size.aligned_size,
      )),
      SimpleType::Vector(t) => t.get_size(platform_size),
    }
  }
}

impl GetSize for VectorType {
  fn get_size(&self, platform_size: PlarformWidth) -> Option<u64> {
    let VectorType(t, s) = self;
    t.get_size(platform_size)
      .map(|x| size_align(x * s, platform_size.aligned_size))
  }
}

impl GetSize for ArrayType {
  fn get_size(&self, platform_size: PlarformWidth) -> Option<u64> {
    let ArrayType(t, s) = self;
    t.get_size(platform_size)
      .map(|x| size_align(x * s, platform_size.aligned_size))
  }
}

fn size_align(i: u64, aligned_size: u64) -> u64 {
  let platform_size = aligned_size;
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
    if self.0 {
      r.sum()
    } else {
      r.map(|x| x.map(|x| size_align(x, platform_size.aligned_size)))
        .sum()
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
    let map_array = [8, 16, 32, 64, 128, 128];
    Some(size_align(
      map_array[self.clone() as usize],
      platform_size.aligned_size,
    ))
  }
}

pub type IsExtern = bool;

pub type IsPublic = bool;

pub type IsAtomic = bool;

#[repr(u8)]
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

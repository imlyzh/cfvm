
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
    Array(Box<Type>, usize),
    Struct(RecordType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SimpleType {
    Int(IntType),
    Float(FloatType),
    Pointer(Box<Type>),
    Vector(Box<SimpleType>, usize), // non include vector
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RecordType {
    pub not_aligned: bool,
    pub record: Vec<(Option<String>, Type)>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionType {
    pub return_type: Box<Type>,
    pub params: Vec<(Option<String>, Type)>,
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
    PpcFp128 = 5,
}


pub trait GetSize {
    fn get_size(&self, platform_size: u8) -> Option<usize>;
}

impl GetSize for Type {
    fn get_size(&self, platform_size: u8) -> Option<usize> {
        if let Type::FirstClassType(t) = self {
            t.get_size(platform_size)
        } else {
            None
        }
    }
}

impl GetSize for FirstClassType {
    fn get_size(&self, platform_size: u8) -> Option<usize> {
        match self {
            FirstClassType::OpaqueType => None,
            FirstClassType::SimpleType(t) => t.get_size(platform_size),
            FirstClassType::Array(t, s) => t.get_size(platform_size).map(|x|x*s),
            FirstClassType::Struct(r) => r.get_size(platform_size),
        }
    }
}

impl GetSize for SimpleType {
    fn get_size(&self, platform_size: u8) -> Option<usize> {
        match self {
            SimpleType::Int(t) => t.get_size(platform_size),
            SimpleType::Float(t) => t.get_size(platform_size),
            SimpleType::Pointer(_) => Some(platform_size as usize),
            SimpleType::Vector(t, s) => t.get_size(platform_size).map(|x|x*s),
        }
    }
}

fn size_align(i: usize, platform_size: u8) -> usize {
    let platform_size  = platform_size as usize;
    let i_mod = i % platform_size;
    if i_mod == 0 {
        i
    } else {
        i + platform_size - i_mod
    }
}

impl GetSize for RecordType {
    fn get_size(&self, platform_size: u8) -> Option<usize> {
        let r = self.record.iter().map(|(_, t)| t.get_size(platform_size));
        if self.not_aligned {
            r.sum()
        } else {
            r.map(|x| x.map(|x| size_align(x,  platform_size))).sum()
        }
    }
}

impl GetSize for IntType {
    fn get_size(&self, _: u8) -> Option<usize> {
        let map_array = [1, 1, 2, 4, 8, 16];
        Some(map_array[*self as usize])
    }
}

impl GetSize for FloatType {
    fn get_size(&self, _: u8) -> Option<usize> {
        let map_array = [1, 2, 4, 8, 16, 16];
        Some(map_array[*self as usize])
    }
}

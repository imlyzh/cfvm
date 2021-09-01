use std::collections::HashMap;
use std::sync::Arc;

use sexpr_ir::gast::GAst;
use sexpr_ir::gast::Handle;
use sexpr_process::capture::{Capture, Catch};

use crate::nodes::*;
use crate::nodes::handles::*;
use crate::nodes::types::*;
use crate::patterns::*;


trait FromGast {
    type Target;
    fn from_gast(ast: &GAst) -> Result<Self::Target, ()>;
}

impl FromGast for Module {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let mut name: Option<Handle<String>> = None;
        let mut type_defs: HashMap<Handle<String>, TypeDef> = HashMap::new();
        let mut constant_defs: HashMap<Handle<String>, ConstantDef> = HashMap::new();
        let mut variable_defs: HashMap<Handle<String>, VariableDef> = HashMap::new();
        let mut functions: HashMap<Handle<String>, FunctionDef> = HashMap::new();
        let mut function_decls: HashMap<Handle<String>, FunctionDecl> = HashMap::new();
        let defines = if let Ok(c) = NAMED_MODULE.catch(ast) {
            let c: HashMap<&str, &Capture> = c
            .iter()
            .map(|(s, c)| ((s.0).as_str(), c))
            .collect();
            let n = c.get("name").unwrap().get_one().unwrap();
            let n = symbol_from_gast(n)?;
            name = Some(n);
            c.get("defines").unwrap().get_many().unwrap().clone()
        } else {
            let c = MODULE.catch(ast).map_err(|_| ())?;
            let c: HashMap<&str, &Capture> = c
            .iter()
            .map(|(s, c)| ((s.0).as_str(), c))
            .collect();
            c.get("defines").unwrap().get_many().unwrap().clone()
        };
        for define in defines {
            match module_item_from_gast(&define)? {
                ModuleItem::Functions(x) => {
                    functions.insert(x.name.0.clone(), x);
                },
                ModuleItem::FunctionDecls(x) => {
                    function_decls.insert(x.name.0.clone(), x);
                },
                ModuleItem::TypeDefs(x) => {
                    type_defs.insert(x.name.0.clone(), x);
                },
                ModuleItem::ConstantDef(value ) => {
                    constant_defs.insert(value.1.0.clone(), value);
                },
                ModuleItem::VariableDef(value ) => {
                    variable_defs.insert(value.1.0.clone(), value);
                },
            }
        }
        Ok(Module {
            name,
            type_defs,
            constant_defs,
            variable_defs,
            functions,
            function_decls,
        })
    }
}

enum ModuleItem {
    TypeDefs(TypeDef),
    ConstantDef(ConstantDef),
    VariableDef(VariableDef),
    Functions(FunctionDef),
    FunctionDecls(FunctionDecl),
}

fn module_item_from_gast(ast: &GAst) -> Result<ModuleItem, ()> {
    if let Ok(r) = FunctionDecl::from_gast(ast) {
        Ok(ModuleItem::FunctionDecls(r))
    } else if let Ok(r) = TypeDef::from_gast(ast) {
        Ok(ModuleItem::TypeDefs(r))
    } else if let Ok(r) = ConstantDef::from_gast(ast) {
        Ok(ModuleItem::ConstantDef(r))
    }else if let Ok(r) = VariableDef::from_gast(ast) {
        Ok(ModuleItem::VariableDef(r))
    } else {
        Err(())
    }
}

impl FromGast for ConstantDef {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let r = GLOBAL_CONST_DEF.catch(ast).map_err(|_| ())?;
        let r: HashMap<&str, &Capture> = r
            .iter()
            .map(|(s, c)| ((s.0).as_str(), c))
            .collect();
        let attrs = r.get("attrs").unwrap().get_one().unwrap();
        let attrs = Attris::from_gast(attrs)?;
        let attrs = attrs.have_and_only_have_flags(&["public"]).ok_or(())?;
        let is_public = matches!(&attrs[..], [true]);

        let name = r.get("global-name").unwrap().get_one().unwrap();
        let name = DefineSymbol::from_gast(name)?;

        let type_ = r.get("type").unwrap().get_one().unwrap();
        let type_ = Type::from_gast(type_)?;

        let expr = r.get("expr").unwrap().get_one().unwrap();
        // let expr = GlobalValue::from_gast(expr)?;

        Ok(ConstantDef(is_public, name, type_, todo!()))
    }
}

impl FromGast for VariableDef {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let r = GLOBAL_VARIABLE_DEF.catch(ast).map_err(|_| ())?;
        let r: HashMap<&str, &Capture> = r
            .iter()
            .map(|(s, c)| ((s.0).as_str(), c))
            .collect();
        let attrs = r.get("attrs").unwrap().get_one().unwrap();
        let attrs = Attris::from_gast(attrs)?;
        let attrs = attrs.have_and_only_have_flags(&["public"]).ok_or(())?;
        let is_public = matches!(&attrs[..], [true]);

        let name = r.get("global-name").unwrap().get_one().unwrap();
        let name = DefineSymbol::from_gast(name)?;

        let type_ = r.get("type").unwrap().get_one().unwrap();
        let type_ = Type::from_gast(type_)?;

        let expr = r.get("expr").unwrap().get_many().unwrap();
        // let expr = GlobalValue::from_gast(expr)?;

        Ok(VariableDef(is_public, name, type_, todo!()))
    }
}

impl FromGast for GlobalValue {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        todo!()
    }
}

impl FromGast for FunctionDecl {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let r = FUNCTION_DECL.catch(ast).map_err(|_| ())?;
        let r: HashMap<&str, &Capture> = r
            .iter()
            .map(|(s, c)| ((s.0).as_str(), c))
            .collect();
        let attrs = r.get("attrs").unwrap().get_one().unwrap();
        let attrs = Attris::from_gast(attrs)?;
        let attrs = attrs.have_and_only_have_flags(&["public"]).ok_or(())?;
        let is_public = matches!(&attrs[..], [true]);

        let name = r.get("name").unwrap().get_one().unwrap();
        let name = DefineSymbol::from_gast(name)?;

        let ret_type = r.get("ret-type").unwrap().get_one().unwrap();
        let ret_type = Type::from_gast(ret_type)?;

        let param_type = r.get("param-type").unwrap().get_one().unwrap();
        let param_type = param_type_from_gast(param_type)?;

        let header = FunctionType {
            return_type: Box::new(ret_type),
            params: param_type
        };
        Ok(FunctionDecl {is_public, name, header})
    }
}

impl FromGast for TypeDef {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let r = TYPE_DEF.catch(ast).map_err(|_| ())?;
        let r: HashMap<&str, &Capture> = r
            .iter()
            .map(|(s, c)| ((s.0).as_str(), c))
            .collect();
        let attrs = r.get("attrs").unwrap().get_one().unwrap();
        let attrs = Attris::from_gast(attrs)?;
        let attrs = attrs.have_and_only_have_flags(&["public"]).ok_or(())?;
        let is_public = matches!(&attrs[..], [true]);

        let name = r.get("name").unwrap().get_one().unwrap();
        let name = DefineSymbol::from_gast(name)?;

        let type_ = r.get("type").unwrap().get_one().unwrap();
        let type_ = Type::from_gast(type_)?;

        Ok(TypeDef { is_public, name, type_ })
    }
}

impl FromGast for Type {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        if is_void(ast).is_some() {
            return Ok(Type::Void);
        }
        if let Ok(t) = FirstClassType::from_gast(ast) {
            return Ok(Type::FirstClassType(t));
        }
        FunctionType::from_gast(ast).map(Type::FunctionType)
    }
}

impl FromGast for FirstClassType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        if is_opaque(ast).is_some() {
            return Ok(FirstClassType::OpaqueType);
        }
        if let Ok(r) = SimpleType::from_gast(ast) {
            return Ok(FirstClassType::SimpleType(r));
        }
        if let Ok(r) = ArrayType::from_gast(ast) {
            return Ok(FirstClassType::Array(r));
        }
        RecordType::from_gast(ast).map(FirstClassType::Record)
    }
}

impl FromGast for SimpleType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        if let Ok(x) = IntType::from_gast(ast) {
            return Ok(SimpleType::Int(x));
        } else if let Ok(x) = FloatType::from_gast(ast) {
            return Ok(SimpleType::Float(x));
        } else if let Ok(x) = PointerType::from_gast(ast) {
            return Ok(SimpleType::Pointer(x));
        } else {
            VectorType::from_gast(ast).map(SimpleType::Vector)
        }
    }
}

impl FromGast for IntType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let s = symbol_from_gast(ast)?;
        let r = if s.as_str() == "i1" {
            IntType::I1
        } else if s.as_str() == "i8" {
            IntType::I8
        } else if s.as_str() == "i16" {
            IntType::I16
        } else if s.as_str() == "i32" {
            IntType::I32
        } else if s.as_str() == "i64" {
            IntType::I64
        } else if s.as_str() == "i128" {
            IntType::I128
        } else {
            return Err(());
        };
        Ok(r)
    }
}

impl FromGast for FloatType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let s = symbol_from_gast(ast)?;
        let r = if s.as_str() == "f8" {
            FloatType::F8
        } else if s.as_str() == "f16" {
            FloatType::F16
        } else if s.as_str() == "f32" {
            FloatType::F32
        } else if s.as_str() == "f64" {
            FloatType::F64
        } else if s.as_str() == "f128" {
            FloatType::F128
        } else if s.as_str() == "ppcf128" {
            FloatType::PpcF128
        } else {
            return Err(());
        };
        Ok(r)
    }
}

impl FromGast for PointerType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let r = POINTER_TYPE
            .catch(ast)
            .map_err(|_| ())?;
        let (_, r) = r.first().unwrap();
        let r = r.get_one().unwrap();
        let r = Type::from_gast(r)?;
        Ok(PointerType(Box::new(r)))
    }
}

impl FromGast for VectorType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let r = VECTOR_TYPE
            .catch(ast)
            .map_err(|_| ())?;
        let r: HashMap<&str, &Capture> = r
            .iter()
            .map(|(s, c)| ((s.0).as_str(), c))
            .collect();
        let type_ = r.get("type").unwrap().get_one().unwrap();
        let type_ = SimpleType::from_gast(type_)?;
        let number = r.get("number").unwrap().get_one().unwrap();
        let number = uint_from_gast(number)?;
        Ok(VectorType(Box::new(type_), number))
    }
}

impl FromGast for ArrayType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let r = ARRAY_TYPE
            .catch(ast)
            .map_err(|_| ())?;
        let r: HashMap<&str, &Capture> = r
            .iter()
            .map(|(s, c)| ((s.0).as_str(), c))
            .collect();
        let type_ = r.get("type").unwrap().get_one().unwrap();
        let type_ = Type::from_gast(type_)?;
        let number = r.get("number").unwrap().get_one().unwrap();
        let number = uint_from_gast(number)?;
        Ok(ArrayType(Box::new(type_), number))
    }
}

fn record_line_from_gast(ast: &GAst) -> Result<(Option<Arc<String>>, Type), ()> {
    let r = RECORD_LINE
        .catch(ast)
        .map_err(|_| ())?;
    let r: HashMap<&str, &Capture> = r
        .iter()
        .map(|(s, c)| ((s.0).as_str(), c))
        .collect();
    let type_ = r.get("type").unwrap().get_one().unwrap();
    let type_ = Type::from_gast(type_)?;
    let name = r.get("name").unwrap().get_many().unwrap();
    if name.len() > 1 {
        Err(())
    } else if name.len() == 1 {
        let name = symbol_from_gast(name.first().unwrap())?;
        Ok((Some(name), type_))
    } else {
        Ok((None, type_))
    }
}

impl FromGast for RecordType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let not_aligned: bool;
        let r = if let Ok(r) = RECORD_TYPE.catch(ast) {
            not_aligned = true;
            r
        } else {
            let r = ALIGNED_RECORD_TYPE.catch(ast).map_err(|_| ())?;
            not_aligned = false;
            r
        };
        let r = &r.first().unwrap().1;
        let record: Result<Vec<_>, ()> = r.get_many().unwrap().into_iter().map(record_line_from_gast).collect();
        let record = record?;
        Ok(RecordType { not_aligned, record })
    }
}

fn param_pair_from_gast(ast: &GAst) -> Result<(Option<LocalSymbol>, Type), ()> {
    let r = FUNCTION_TYPE
        .catch(ast)
        .map_err(|_| ())?;
    let r: HashMap<&str, &Capture> = r
        .iter()
        .map(|(s, c)| ((s.0).as_str(), c))
        .collect();
    let type_ = r
        .get("type").unwrap()
        .get_one().unwrap();
    let type_ = Type::from_gast(type_)?;
    let name = r
        .get("name").unwrap()
        .get_many().unwrap()
        .first();
    let name = if let Some(n) = name {
        Some(LocalSymbol::from_gast(n)?)
    } else {
        None
    };
    Ok((name, type_))
}



fn param_type_from_gast(ast: &GAst) -> Result<Vec<(Option<LocalSymbol>, Type)>, ()> {
    let param_type = PARAMS_TYPE.catch(ast).map_err(|_| ())?;
    let param_types = param_type.first().unwrap().1.get_many().unwrap();
    param_types.iter().map(param_pair_from_gast).collect()
}

impl FromGast for FunctionType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let r = FUNCTION_TYPE.catch(ast).map_err(|_| ())?;
        let r: HashMap<&str, &Capture> = r.iter().map(|(s, c)| ((s.0).as_str(), c)).collect();
        let ret_type = r.get("ret-type").unwrap().get_one().unwrap();
        let param_type = r.get("param-type").unwrap().get_one().unwrap();
        let ret_type = Type::from_gast(ret_type)?;
        let param_type = param_type_from_gast(param_type)?;
        let r = FunctionType {
            return_type: Box::new(ret_type),
            params: param_type,
        };
        Ok(r)
    }
}

fn is_void(ast: &GAst) -> Option<()> {
    if ast.get_const()?.get_sym()?.0.as_str() == "void" {
        Some(())
    } else {
        None
    }
}

fn is_opaque(ast: &GAst) -> Option<()> {
    if ast.get_const()?.get_sym()?.0.as_str() == "opaque" {
        Some(())
    } else {
        None
    }
}

impl FromGast for LocalSymbol {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let name = symbol_from_gast(ast)?;
        if name.chars().next().unwrap() != '%' {
            return Err(());
        }
        Ok(LocalSymbol(name))
    }
}

impl FromGast for GlobalSymbol {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let name = symbol_from_gast(ast)?;
        if name.chars().next().unwrap() != '@' {
            return Err(());
        }
        Ok(GlobalSymbol(name))
    }
}

impl FromGast for LabelSymbol {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let name = symbol_from_gast(ast)?;
        if name.chars().next().unwrap() != ':' {
            return Err(());
        }
        Ok(LabelSymbol(name))
    }
}

impl FromGast for DefineSymbol {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        symbol_from_gast(ast).map(DefineSymbol)
    }
}

impl FromGast for Attris {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let r = ATTRIS.catch(ast).map_err(|_| ())?;
        let r = r.first().unwrap().1.get_many().unwrap();
        let r: Result<Vec<_>, _> = r.iter().map(symbol_from_gast).collect();
        let r = r?;
        Ok(Attris(r))
    }
}

fn symbol_from_gast(ast: &GAst) -> Result<Arc<String>, ()> {
    let name = ast.get_const().ok_or(())?.get_sym().ok_or(())?;
    Ok(name.0.clone())
}

fn uint_from_gast(ast: &GAst) -> Result<u64, ()> {
    ast.get_const().ok_or(())?.get_uint().ok_or(())
}

fn int_from_gast(ast: &GAst) -> Result<i64, ()> {
    ast.get_const().ok_or(())?.get_int().ok_or(())
}

fn float_from_gast(ast: &GAst) -> Result<f64, ()> {
    ast.get_const().ok_or(())?.get_float().ok_or(())
}

fn bool_from_gast(ast: &GAst) -> Result<bool, ()> {
    ast.get_const().ok_or(())?.get_bool().ok_or(())
}
use std::collections::HashMap;

use sexpr_ir::gast::GAst;
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
        todo!()
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

impl FromGast for Type {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        if is_void(ast).is_some() {
            return Ok(Type::Void);
        }
        if let Ok(t) = FirstClassType::from_gast(ast) {
            return Ok(Type::FirstClassType(t));
        }
        if let Ok(t) = FunctionType::from_gast(ast) {
            return Ok(Type::FunctionType(t));
        }
        Err(())
    }
}

impl FromGast for FirstClassType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        if is_opaque(ast).is_some() {
            return Ok(FirstClassType::OpaqueType);
        }
        todo!()
    }
}

impl FromGast for SimpleType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        todo!()
    }
}

impl FromGast for ArrayType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        todo!()
    }
}

impl FromGast for RecordType {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        todo!()
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


impl FromGast for LocalSymbol {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let name = ast.get_const().ok_or(())?.get_sym().ok_or(())?;
        let name = name.0.clone();
        if name.chars().next().unwrap() != '%' {
            return Err(());
        }
        Ok(LocalSymbol(name))
    }
}

impl FromGast for GlobalSymbol {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let name = ast.get_const().ok_or(())?.get_sym().ok_or(())?;
        let name = name.0.clone();
        if name.chars().next().unwrap() != '@' {
            return Err(());
        }
        Ok(GlobalSymbol(name))
    }
}

impl FromGast for LabelSymbol {
    type Target = Self;

    fn from_gast(ast: &GAst) -> Result<Self::Target, ()> {
        let name = ast.get_const().ok_or(())?.get_sym().ok_or(())?;
        let name = name.0.clone();
        if name.chars().next().unwrap() != ':' {
            return Err(());
        }
        Ok(LabelSymbol(name))
    }
}
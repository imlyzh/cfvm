///# Get Data dependence
use std::collections::HashSet;

use crate::{
  control::{Control, ControlInst, If},
  data::*,
  effect::*,
  function::*,
};

pub trait GetDataDep {
  fn get_data_dep(&self) -> Vec<Data>;
}

impl GetDataDep for Func {
  fn get_data_dep(&self) -> Vec<Data> {
    let mut datas: Vec<Data> = self
      .effects
      .iter()
      .flat_map(|x| unsafe { x.as_ref() }.get_data_dep())
      .collect();
    datas.extend(
      self
        .controls
        .iter()
        .flat_map(|x| unsafe { x.as_ref() }.get_data_dep()),
    );

    // Graph Propagation Process

    let mut old_datas: Option<HashSet<Data>> = None;
    let mut new_datas: Option<HashSet<Data>> = Some(HashSet::from_iter(datas));
    loop {
      if new_datas == old_datas {
        return new_datas.unwrap().into_iter().collect();
      }
      let tmp_datas = Some(HashSet::from_iter(
        new_datas
          .as_ref()
          .unwrap()
          .iter()
          .flat_map(Data::get_data_dep),
      ));
      old_datas = new_datas;
      new_datas = tmp_datas;
    }
  }
}

impl GetDataDep for Data {
  fn get_data_dep(&self) -> Vec<Data> {
    match self.data {
      DataInst::Const(_, _) | DataInst::Alloc(_) | DataInst::Input(_) => vec![],
      DataInst::TypeCast(o) => unsafe { o.as_ref() }.get_data_dep(),
      DataInst::PriOp(o) => unsafe { o.as_ref() }.get_data_dep(),
      DataInst::BinOp(o) => unsafe { o.as_ref() }.get_data_dep(),
      DataInst::AddrOp(o) => unsafe { o.as_ref() }.get_data_dep(),
      DataInst::Phi(o) => unsafe { o.as_ref() }.get_data_dep(),
      DataInst::Effect(o) => unsafe { o.as_ref() }.get_data_dep(),
    }
  }
}

impl GetDataDep for TypeCast {
  fn get_data_dep(&self) -> Vec<Data> {
    vec![self.data.clone()]
  }
}

impl GetDataDep for PriOp {
  fn get_data_dep(&self) -> Vec<Data> {
    match self {
      PriOp::Trunc(d, _)
      | PriOp::ZExt(d, _)
      | PriOp::SExt(d, _)
      | PriOp::FTrunc(d, _)
      | PriOp::FExt(d, _) => vec![d.clone()],
    }
  }
}

impl GetDataDep for BinOp {
  fn get_data_dep(&self) -> Vec<Data> {
    vec![self.data0.clone(), self.data1.clone()]
  }
}

impl GetDataDep for AddrOp {
  fn get_data_dep(&self) -> Vec<Data> {
    match self {
      AddrOp::RuntimeArrayItem(a, b) => vec![a.clone(), b.clone()],
      AddrOp::ArrayItem(a, _) | AddrOp::TupleItem(a, _) => vec![a.clone()],
    }
  }
}

impl GetDataDep for Phi {
  fn get_data_dep(&self) -> Vec<Data> {
    self.data.iter().flat_map(Data::get_data_dep).collect()
  }
}

impl GetDataDep for Effect {
  fn get_data_dep(&self) -> Vec<Data> {
    let r = match self.effect.clone() {
      EffectInst::Read { ptr } => vec![ptr],
      EffectInst::Write { ptr, value } => vec![ptr, value],
      EffectInst::Call { args } => args,
      EffectInst::IndirectCall { func, mut args } => {
        args.push(func);
        args
      },
    };
    /*
    let append: Vec<_> = self
      .effect_source
      .iter()
      .flat_map(|x| unsafe { x.as_ref() }.get_data_dep())
      .collect();
    //  */
    r
  }
}

impl GetDataDep for Control {
  fn get_data_dep(&self) -> Vec<Data> {
    match self.control.clone() {
      ControlInst::If(i) => vec![i.0],
      ControlInst::Return(r) => vec![r],
      ControlInst::Jump | ControlInst::Unreachable => vec![],
    }
  }
}

impl GetDataDep for If {
  fn get_data_dep(&self) -> Vec<Data> {
    self.0.get_data_dep()
  }
}

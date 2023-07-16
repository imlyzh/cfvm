use std::{collections::HashSet, ptr::NonNull, vec};

use super::get_data_dep::GetDataDep;
use crate::{
  control::{Control, ControlInst, If, Region},
  data::*,
  effect::*,
  function::*,
};

pub trait GetRegions {
  fn get_regions(&self) -> Vec<NonNull<Region>>;
}

impl GetRegions for Func {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    let controls: &Vec<NonNull<Control>> = &self.controls;
    let effects: &Vec<NonNull<Effect>> = &self.effects;
    let datas: &Vec<Data> = &self.get_data_dep();

    body2regions(controls, effects, datas)
  }
}

pub fn body2regions(
  controls: &Vec<NonNull<Control>>,
  effects: &Vec<NonNull<Effect>>,
  datas: &Vec<Data>,
) -> Vec<NonNull<Region>> {
  let mut regions: Vec<NonNull<Region>> = controls
    .iter()
    .flat_map(|x| unsafe { x.as_ref() }.get_regions())
    .collect();
  regions.extend(
    effects
      .iter()
      .flat_map(|x| unsafe { x.as_ref() }.get_regions()),
  );
  regions.extend(datas.iter().flat_map(Data::get_regions));

  HashSet::<NonNull<Region>>::from_iter(regions.into_iter())
    .into_iter()
    .collect()
}

impl GetRegions for Data {
  #[inline(always)]
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    vec![self.region_source]
  }
}

impl GetRegions for Effect {
  #[inline(always)]
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    vec![self.region_source]
  }
}

impl GetRegions for Control {
  #[inline(always)]
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    vec![self.region_source]
  }
}

impl GetRegions for DataInst {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    match self {
      DataInst::Const(_, _) | DataInst::Alloc(_) | DataInst::Input(_) => vec![],
      DataInst::TypeCast(d) => unsafe { d.as_ref() }.get_regions(),
      DataInst::PriOp(d) => unsafe { d.as_ref() }.get_regions(),
      DataInst::BinOp(d) => unsafe { d.as_ref() }.get_regions(),
      DataInst::AddrOp(d) => unsafe { d.as_ref() }.get_regions(),
      DataInst::Phi(_d) => vec![],
      // DataInst::Phi(d) => unsafe { d.as_ref() }.get_regions(),
      DataInst::Effect(d) => unsafe { d.as_ref() }.get_regions(),
    }
  }
}

impl GetRegions for PriOp {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    match self {
      PriOp::Trunc(d, _)
      | PriOp::ZExt(d, _)
      | PriOp::SExt(d, _)
      | PriOp::FTrunc(d, _)
      | PriOp::FExt(d, _) => d.get_regions(),
    }
  }
}

impl GetRegions for BinOp {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    let mut r = self.data0.get_regions();
    r.append(&mut self.data1.get_regions());
    r
  }
}

impl GetRegions for AddrOp {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    match self {
      AddrOp::RuntimeArrayItem(data0, data1) => {
        let mut r = data0.get_regions();
        r.append(&mut data1.get_regions());
        r
      },
      AddrOp::ArrayItem(data, _) | AddrOp::TupleItem(data, _) => data.get_regions(),
    }
  }
}

impl GetRegions for TypeCast {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    self.data.get_regions()
  }
}

impl GetRegions for Phi {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    self.data.iter().flat_map(Data::get_regions).collect()
  }
}

impl GetRegions for EffectInst {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    match self {
      EffectInst::Read { ptr } => ptr.get_regions(),
      EffectInst::Write { ptr, value } => {
        let mut r = ptr.get_regions();
        r.append(&mut value.get_regions());
        r
      },
      EffectInst::Call { args } => args.iter().flat_map(Data::get_regions).collect(),
      EffectInst::IndirectCall { func, args } => {
        let mut r: Vec<_> = func.get_regions();
        r.append(&mut args.iter().flat_map(Data::get_regions).collect());
        r
      },
    }
  }
}

impl GetRegions for ControlInst {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    match self {
      ControlInst::If(i) => i.get_regions(),
      ControlInst::Return(d) => d.get_regions(),
      ControlInst::Jump | ControlInst::Unreachable => vec![],
    }
  }
}

impl GetRegions for If {
  fn get_regions(&self) -> Vec<NonNull<Region>> {
    self.0.get_regions()
  }
}
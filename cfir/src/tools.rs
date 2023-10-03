use std::collections::HashMap;

use crate::{
  op::{OpHand, Space},
  symbol::Symbol,
  value::Value,
};

pub fn relinking(ops: &Space) -> Space {
  let mut record = HashMap::new();
  for op in ops {
    for (offset, name) in op.as_ref().borrow().defs.iter().enumerate() {
      record.insert(name.clone(), Value::Use(op.clone(), offset));
    }
  }
  name_replace(&record, ops)
}

pub fn name_replace(record: &HashMap<Symbol, Value>, ops: &Space) -> Space {
  ops.iter().map(|op| name_replace_op(record, op)).collect()
}

pub fn name_replace_op(record: &HashMap<Symbol, Value>, op: &OpHand) -> OpHand {
  let mut op = op.as_ref().borrow().clone();
  op.uses = op
    .uses
    .into_iter()
    .map(|v| match &v {
      Value::Input(sym) => {
        if let Some(record_v) = record.get(sym) {
          record_v.clone()
        } else {
          v
        }
      },
      _ => v,
    })
    .collect();
  OpHand::new(op)
}

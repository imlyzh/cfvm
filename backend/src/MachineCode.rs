use cfvm_common::constant::Symbol;

pub type MachineCodeList = Vec<MachineCode>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineCode (pub Symbol, Vec<Param>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Param {
  // literal symbol reference
  LitSymRef(Symbol),
  Reg(u64),
  Lit1(u8),
  Lit2(u16),
  Lit4(u32),
  Lit8(u64),
}
pub mod reg_alloc;

pub mod rv64;
pub mod x86_64;
pub mod aarch64;

use libemei::insts::inst_dump_buf::{self, InstBuffer};

use crate::cfir::graphir::{basicblock::BasicBlockDef, instruction::Instruction};



#[derive(Debug, Clone)]
pub struct InstStream(pub Vec<Instruction>);

pub trait Streamify {
    fn streamify(self) -> InstStream;
}

impl Streamify for BasicBlockDef {
    fn streamify(self) -> InstStream {
        InstStream(self.instructions.read().unwrap().iter().map(|x| x.read().unwrap().clone()).collect())
    }
}


pub trait CodeGen {
    fn gen_code(&self, buf: &InstBuffer);
}

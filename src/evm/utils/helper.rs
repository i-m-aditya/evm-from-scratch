use std::collections::HashMap;

use crate::evm::{opcodes, utils::context::ExecutionContext};

pub type OpcodeResult = Result<Option<Vec<u8>>, anyhow::Error>;
pub type Opcode = Box<dyn Fn(&mut ExecutionContext) -> OpcodeResult>;
pub type Opcodes = HashMap<u8, Opcode>;
pub fn get_opcodes() -> Opcodes {
    let mut opcodes: Opcodes = HashMap::new();

    opcodes.insert(0x00, Box::new(opcodes::stop_and_arithmetic::stop));

    opcodes.insert(0x5f, Box::new(opcodes::push::push));
    opcodes.insert(0x60, Box::new(opcodes::push::push));
    opcodes.insert(0x61, Box::new(opcodes::push::push));
    opcodes.insert(0x62, Box::new(opcodes::push::push));
    opcodes.insert(0x63, Box::new(opcodes::push::push));
    opcodes.insert(0x64, Box::new(opcodes::push::push));
    opcodes.insert(0x65, Box::new(opcodes::push::push));
    opcodes.insert(0x66, Box::new(opcodes::push::push));
    opcodes.insert(0x67, Box::new(opcodes::push::push));
    opcodes.insert(0x68, Box::new(opcodes::push::push));
    opcodes.insert(0x69, Box::new(opcodes::push::push));
    opcodes.insert(0x6a, Box::new(opcodes::push::push));
    opcodes.insert(0x6b, Box::new(opcodes::push::push));
    opcodes.insert(0x6c, Box::new(opcodes::push::push));
    opcodes.insert(0x6d, Box::new(opcodes::push::push));
    opcodes.insert(0x6e, Box::new(opcodes::push::push));
    opcodes.insert(0x6f, Box::new(opcodes::push::push));
    opcodes.insert(0x70, Box::new(opcodes::push::push));
    opcodes.insert(0x71, Box::new(opcodes::push::push));
    opcodes.insert(0x72, Box::new(opcodes::push::push));
    opcodes.insert(0x73, Box::new(opcodes::push::push));
    opcodes.insert(0x74, Box::new(opcodes::push::push));
    opcodes.insert(0x75, Box::new(opcodes::push::push));
    opcodes.insert(0x76, Box::new(opcodes::push::push));
    opcodes.insert(0x77, Box::new(opcodes::push::push));
    opcodes.insert(0x78, Box::new(opcodes::push::push));
    opcodes.insert(0x79, Box::new(opcodes::push::push));
    opcodes.insert(0x7a, Box::new(opcodes::push::push));
    opcodes.insert(0x7b, Box::new(opcodes::push::push));
    opcodes.insert(0x7c, Box::new(opcodes::push::push));
    opcodes.insert(0x7d, Box::new(opcodes::push::push));
    opcodes.insert(0x7e, Box::new(opcodes::push::push));
    opcodes.insert(0x7f, Box::new(opcodes::push::push));

    opcodes
}

use std::collections::HashMap;

use primitive_types::U256;

use crate::evm::{opcodes, utils::context::ExecutionContext};

pub type OpcodeResult = Result<Option<Vec<u8>>, anyhow::Error>;
pub type Opcode = Box<dyn Fn(&mut ExecutionContext) -> OpcodeResult>;
pub type Opcodes = HashMap<u8, Opcode>;

pub fn pop_n(ctx: &mut ExecutionContext, n: usize) -> Vec<U256> {
    let mut popped_items: Vec<U256> = vec![];

    (0..n).for_each(|_| {
        if let Some(item) = ctx.stack.pop() {
            popped_items.push(item);
        }
    });

    popped_items
}

pub fn get_opcodes() -> Opcodes {
    let mut opcodes: Opcodes = HashMap::new();

    opcodes.insert(0x00, Box::new(opcodes::stop_and_arithmetic::stop));
    opcodes.insert(0x01, Box::new(opcodes::stop_and_arithmetic::add));
    opcodes.insert(0x02, Box::new(opcodes::stop_and_arithmetic::mul));
    opcodes.insert(0x03, Box::new(opcodes::stop_and_arithmetic::sub));
    opcodes.insert(0x04, Box::new(opcodes::stop_and_arithmetic::div));
    // opcodes.insert(0x05, Box::new(opcodes::stop_and_arithmetic::sdiv));
    opcodes.insert(0x06, Box::new(opcodes::stop_and_arithmetic::modd));
    // opcodes.insert(0x07, Box::new(opcodes::stop_and_arithmetic::smod));
    opcodes.insert(0x08, Box::new(opcodes::stop_and_arithmetic::addmod));
    opcodes.insert(0x09, Box::new(opcodes::stop_and_arithmetic::mulmod));
    opcodes.insert(0x0a, Box::new(opcodes::stop_and_arithmetic::exponent));
    // opcodes.insert(0x0b, Box::new(opcodes::stop_and_arithmetic::signextend));

    opcodes.insert(0x50, Box::new(opcodes::push::pop));

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

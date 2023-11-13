use anyhow::Ok;

use crate::evm::utils::{context::ExecutionContext, helper::OpcodeResult};

pub fn stop(_ctx: &mut ExecutionContext) -> OpcodeResult {
    Ok(Some(Vec::new()))
}

pub fn add(_ctx: &mut ExecutionContext) -> OpcodeResult {
    let a = _ctx.stack.pop().unwrap();
    let b = _ctx.stack.pop().unwrap();
    println!("Hello");
    println!("{:#X}", a);
    println!("{:#X}", b);

    let (sum, _overflown) = a.overflowing_add(b);

    _ctx.stack.push(sum);
    _ctx.pc += 1;
    Ok(None)
}

pub fn mul(_ctx: &mut ExecutionContext) -> OpcodeResult {
    let a = _ctx.stack.pop().unwrap();
    let b = _ctx.stack.pop().unwrap();
    println!("Hello");
    println!("{:#X}", a);
    println!("{:#X}", b);

    let (sum, _overflown) = a.overflowing_mul(b);

    _ctx.stack.push(sum);
    _ctx.pc += 1;
    Ok(None)
}

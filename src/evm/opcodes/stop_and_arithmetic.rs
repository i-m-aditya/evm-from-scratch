use std::ops::Div;

use anyhow::Ok;
use primitive_types::U256;

use crate::evm::utils::{
    context::ExecutionContext,
    helper::{pop_n, OpcodeResult},
};

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

    println!("Val1, {:#X}", a);
    println!("Val2, {:#X}", b);

    let (sum, _overflown) = a.overflowing_mul(b);

    _ctx.stack.push(sum);
    _ctx.pc += 1;
    Ok(None)
}

pub fn sub(_ctx: &mut ExecutionContext) -> OpcodeResult {
    let a = _ctx.stack.pop().unwrap();
    let b = _ctx.stack.pop().unwrap();
    println!("Hello");
    println!("{:#X}", a);
    println!("{:#X}", b);

    let (sum, _overflown) = a.overflowing_sub(b);

    _ctx.stack.push(sum);
    _ctx.pc += 1;
    Ok(None)
}
pub fn div(_ctx: &mut ExecutionContext) -> OpcodeResult {
    let a = _ctx.stack.pop().unwrap();
    let b = _ctx.stack.pop().unwrap();
    println!("Hello");
    println!("{:#X}", a);
    println!("{:#X}", b);

    if b == U256::zero() {
        _ctx.stack.push(U256::from(0));
        _ctx.pc += 1;
        return Ok(None);
    }
    let result = a.div(b);

    _ctx.stack.push(result);
    _ctx.pc += 1;
    Ok(None)
}

pub fn modd(_ctx: &mut ExecutionContext) -> OpcodeResult {
    let items = pop_n(_ctx, 2);

    if items[1] == U256::zero() {
        _ctx.pc += 1;
        _ctx.stack.push(U256::from(0));
        return Ok(None);
    }
    let val = items[0] % items[1];
    _ctx.pc += 1;
    _ctx.stack.push(val);

    Ok(None)
}

pub fn addmod(_ctx: &mut ExecutionContext) -> OpcodeResult {
    let _ = add(_ctx);
    modd(_ctx)
}

pub fn mulmod(_ctx: &mut ExecutionContext) -> OpcodeResult {
    // let _ = mul(_ctx);
    // println!("Valllllllll : {:#X}", _ctx.stack.last().unwrap());
    // modd(_ctx)\

    let items = pop_n(_ctx, 3);

    // println!("Itemssssss : {:?}", items);

    let val512 = items[0].full_mul(items[1]);

    let result = val512.checked_rem(items[2].into()).unwrap();

    _ctx.stack.push(result.try_into().unwrap_or(U256::zero()));
    _ctx.pc += 1;
    Ok(None)
}

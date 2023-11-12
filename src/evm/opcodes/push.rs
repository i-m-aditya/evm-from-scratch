use primitive_types::U256;

use crate::evm::utils::{context::ExecutionContext, helper::OpcodeResult};

pub fn push(_ctx: &mut ExecutionContext) -> OpcodeResult {
    let n: usize = (_ctx.input[_ctx.pc] - 0x5f).try_into().unwrap();

    if n == 0 {
        _ctx.stack.push(U256::from(0));
        _ctx.pc += 1;
        return Ok(None);
    }
    let arr = &_ctx.input[_ctx.pc + 1.._ctx.pc + n + 1];

    for item in arr {
        _ctx.stack.push(U256::from(arr));
    }
    _ctx.pc += n + 1;
    Ok(None)
}
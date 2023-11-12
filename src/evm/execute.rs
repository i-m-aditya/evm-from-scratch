use anyhow::{Error, Ok};

use crate::EvmResult;

use super::utils::{context::ExecutionContext, helper::get_opcodes};

pub fn execute(ctx: &mut ExecutionContext) -> Result<EvmResult, Error> {
    let input_len = ctx.input.len();

    let opcodes = get_opcodes();

    while ctx.pc < input_len {
        let runner = opcodes.get(&ctx.input[ctx.pc]).unwrap();
        let output = runner(ctx).unwrap();

        if let Some(_data) = output {
            break;
        }
    }

    Ok(EvmResult {
        stack: ctx.stack.clone(),
        success: true,
    })
}

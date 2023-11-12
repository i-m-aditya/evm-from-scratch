use anyhow::Ok;

use crate::evm::utils::{context::ExecutionContext, helper::OpcodeResult};

pub fn stop(_ctx: &mut ExecutionContext) -> OpcodeResult {
    Ok(Some(Vec::new()))
}

// pub fn add(_ctx: &mut ExecutionContext) -> OpcodeResult {}

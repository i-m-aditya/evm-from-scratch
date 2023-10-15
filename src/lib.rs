use primitive_types::U256;
mod parse;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    // let mut pc = 0;

    let code = _code.as_ref();

    println!("Code: {:?}", code);

    let opcode = code[0];

    if opcode == 0 {
    } else if opcode >= 95 && opcode <= 127 {
        if code.len() == 1 {
            stack.push(U256::from(0));
        } else {
            let mut pc = 0;
            while pc < code.len() {
                let num_byte_to_push = code[pc] - 95;
                stack.push(U256::from(
                    &code[pc + 1..(pc + 1 + num_byte_to_push as usize)],
                ));
                pc = pc + 1 + num_byte_to_push as usize;
            }
        }
    } else {
        panic!("Not handled")
    }

    // TODO: Implement me

    stack.reverse();
    return EvmResult {
        stack: stack,
        success: true,
    };
}

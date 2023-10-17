use std::result;

use primitive_types::U256;
mod parse;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;

    let code = _code.as_ref();

    println!("Code: {:?}", code);

    let mut stack_len = 0;
    while pc < code.len() {
        if code[pc] == 0 {
            break;
        } else if code[pc] >= 95 && code[pc] <= 127 {
            let byte_cnt = code[pc] - 95;
            if byte_cnt == 0 {
                stack.push(U256::from(0));
                stack_len += 1;
                pc += 1;
                continue;
            }
            stack.push(U256::from(&code[pc + 1..(pc + 1 + byte_cnt as usize)]));
            pc = pc + 1 + byte_cnt as usize;
            stack_len += byte_cnt as usize;
        } else if code[pc] == 80 {
            // println!("Stack len {:?}, {:?}", stack_len, stack.len());
            stack.remove(stack_len - 1);
            stack_len -= 1;
            pc += 1;
        } else if code[pc] == 1 {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();

            let (result, _) = n1.overflowing_add(n2);

            stack.push(result);
            pc += 1;
        } else if code[pc] == 2 {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();

            let (result, _) = n1.overflowing_mul(n2);

            stack.push(result);
            pc += 1;
        } else if code[pc] == 3 {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();

            let (result, _) = n1.overflowing_sub(n2);

            println!("Sub: {:?}, {:?}, {:?}", n1, n2, result);
            stack.push(result);
            pc += 1;
        } else if code[pc] == 4 {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();

            let result = n1.checked_div(n2);
            if result == None {
                stack.push(U256::from(0));
            } else {
                stack.push(result.unwrap());
            }

            println!("Div: {:?}, {:?}, {:?}", n1, n2, result);

            pc += 1;
        } else {
            panic!("Unknown opcode: {:?}", code[pc]);
        }
    }
    // TODO: Implement me

    stack.reverse();
    return EvmResult {
        stack: stack,
        success: true,
    };
}

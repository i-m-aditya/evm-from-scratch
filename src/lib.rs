#[allow(unused_assignments)]
use primitive_types::U256;
use std::ops::Mul;
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
        } else if code[pc] == 6 {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();

            if n2 == U256::zero() {
                stack.push(U256::from(0));
            } else {
                let result = n1 % n2;
                stack.push(result);
            }

            pc += 1;
        } else if code[pc] == 8 {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();
            let n3 = stack.pop().unwrap();

            let (sum, _) = n1.overflowing_add(n2);
            if n3 == U256::zero() {
                stack.push(U256::from(0));
            } else {
                let result = sum % n3;
                stack.push(result);
            }

            pc += 1;
        } else if code[pc] == 9 {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();
            let n3 = stack.pop().unwrap();

            if n3 == U256::zero() {
                stack.push(U256::from(0));
            } else {
                let result = ((n1 % n3).mul(n2 % n3)) % n3;
                stack.push(result);
            }
            pc += 1;
        } else if code[pc] == 10 {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();

            println!("Exp: {:?}, {:?}", n1, n2);
            let (val, _) = n1.overflowing_pow(n2);
            stack.push(val);
            pc += 1;
        } else if code[pc] == 11 {
            let _ = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();

            let format_n = format!("{:x}", n2);

            // check if format_n first char is f or not
            let first_char = format_n.chars().next().unwrap();

            if first_char == 'f' {
                println!("Number negative");

                let result = U256::MAX | n2;
                stack.push(result);
                pc += 1;
            } else {
                println!("Number positive");

                stack.push(n2);
                pc += 1;
            }

            // // let N = U256::from(0x7f);
            // println!("n2: {:?}", n2);
            // let max_val = U256::MAX;

            // let mut test_num = 1;
            // println!("hex: {:x?}", n2.as_u128());

            // while test_num < n2.as_u128() {
            //     println!("hex: {:x?}", test_num);
            //     test_num *= 16;
            // }
            // test_num /= 16;
            // println!("hex: {:x?}", test_num);

            // if test_num <= n2.as_u128() {
            //     println!("Number negative");
            //     test_num *= 16;

            //     let result = max_val | n2;
            //     println!("Result : {:?}", result);
            // } else {
            //     println!("Number positive");
            // }
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

use crate::types::{Instruction, Operand, Register};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Value {
    Exact(i64),
    Input(usize), // Represent the `i`th input to the program
    Unknown,
}

/// Optimzation to remove no op binary instructions.
///
/// These instructions can be removed because they have no effect
/// on the register supplied or the program as a whole.
pub fn propagate_constants(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut new: Vec<Instruction> = vec![];
    let mut registers: [Value; 4] = [Value::Exact(0); 4];

    let mut next_input_idx = 0;
    for instr in instructions {
        if let Instruction::Input(Register(index)) = instr {
            registers[index] = Value::Input(next_input_idx);
            next_input_idx += 1;
        } else {
            let register_idx = instr.destination();
            let left = registers[register_idx];
            let right = match instr.operand().unwrap() {
                Operand::Literal(l) => Value::Exact(l),
                Operand::Register(Register(r)) => registers[r],
            };

            if is_no_op(instr, left, right) {
                continue;
            }

            registers[register_idx] = evaluate_instruction(instr, left, right);
        }

        new.push(instr);
    }

    new
}

/// Check if an instruction is a no-op.
///
/// Example No-ops:
/// _ + 0
/// 0 * _
/// _ * 1
/// 0 / _
/// _ / 1
/// a % b where a < b
/// a eq b where a == 1 == b
/// a eq b where a == 0 != b
fn is_no_op(instr: Instruction, left: Value, right: Value) -> bool {
    match (left, instr, right) {
        (_, Instruction::Add(..), Value::Exact(0)) // _ + 0
        | (Value::Exact(0), Instruction::Mul(..), _) // 0 * _
        | (_, Instruction::Mul(..), Value::Exact(1)) // _ * 1
        | (Value::Exact(0), Instruction::Div(..), _) // 0 / _
        | (_, Instruction::Div(..), Value::Exact(1)) => { // _ / 1
                true
        }
        (Value::Exact(a), Instruction::Mod(..), Value::Exact(b)) => a < b,
        (Value::Exact(a), Instruction::Equal(..), Value::Exact(b)) => {
            ((a == b) && a == 1) || ((a != b) && a == 0)
        },
        _ => false,
    }
}

/// Evaluate an instruction to determine the value of a register.
fn evaluate_instruction(instr: Instruction, left: Value, right: Value) -> Value {
    if let (Value::Exact(left), Value::Exact(right)) = (left, right) {
        let exact_result = match instr {
            Instruction::Input(..) => unreachable!(),
            Instruction::Add(..) => left + right,
            Instruction::Mul(..) => left * right,
            Instruction::Div(..) => left / right,
            Instruction::Mod(..) => left % right,
            Instruction::Equal(..) => {
                if left == right {
                    1
                } else {
                    0
                }
            }
        };

        return Value::Exact(exact_result);
    }

    if matches!(instr, Instruction::Mul(..))
        && (matches!(left, Value::Exact(0)) || matches!(right, Value::Exact(0)))
    {
        return Value::Exact(0);
    }

    Value::Unknown
}

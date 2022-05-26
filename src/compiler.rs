use crate::{
    program::Program,
    types::{Instruction, Operand, Register, Value},
};

/// Optimzation to remove no op binary instructions.
///
/// These instructions can be removed because they have no effect
/// on the register supplied or the program as a whole.
pub fn propagate_constants(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut program = Program::default();
    let mut new: Vec<Instruction> = vec![];
    let mut registers: [Value; 4] = program.initial_registers();

    for instr in instructions {
        if let Instruction::Input(Register(index)) = instr {
            registers[index] = program.new_input_value();
        } else {
            let register_idx = instr.destination();
            let left = registers[register_idx];
            let right = match instr.operand().unwrap() {
                Operand::Literal(lit) => program.new_exact_value(lit),
                Operand::Register(Register(reg)) => registers[reg],
            };

            let old_val = left;
            let new_val = evaluate_instruction(&mut program, instr, left, right);
            registers[register_idx] = new_val;

            if old_val == new_val {
                // No-op
                continue;
            }
        }

        new.push(instr);
    }

    new
}

fn evaluate_add(program: &mut Program, left: Value, right: Value) -> Value {
    match (left, right) {
        (Value::Exact(_, lval), Value::Exact(_, rval)) => program.new_exact_value(lval + rval),
        (_, Value::Exact(_, 0)) => left,  // left + 0 = left
        (Value::Exact(_, 0), _) => right, // 0 + right= right
        _ => program.new_unknown_value(),
    }
}

fn evaluate_mul(program: &mut Program, left: Value, right: Value) -> Value {
    match (left, right) {
        (Value::Exact(_, left), Value::Exact(_, right)) => program.new_exact_value(left * right),
        (_, Value::Exact(_, 0)) | (Value::Exact(_, 0), _) => program.new_exact_value(0), // left * 0 = 0 * right = 0
        (_, Value::Exact(_, 1)) => left,  // left * 1 = left
        (Value::Exact(_, 1), _) => right, // 1 * right = right
        _ => program.new_unknown_value(),
    }
}

fn evaluate_div(program: &mut Program, left: Value, right: Value) -> Value {
    match (left, right) {
        (Value::Exact(_, left), Value::Exact(_, right)) => program.new_exact_value(left / right),
        (Value::Exact(_, 0), _) => left, // 0 / right = 0
        (_, Value::Exact(_, 1)) => left, // left / 1 = left
        _ => program.new_unknown_value(),
    }
}

fn evaluate_mod(program: &mut Program, left: Value, right: Value) -> Value {
    match (left, right) {
        (Value::Exact(_, left), Value::Exact(_, right)) => program.new_exact_value(left % right),
        (Value::Exact(_, 0), _) | (_, Value::Exact(_, 1)) => program.new_exact_value(0),
        _ => program.new_unknown_value(),
    }
}

fn evaluate_eql(program: &mut Program, left: Value, right: Value) -> Value {
    if left == right {
        return program.new_exact_value(1);
    }

    match (left, right) {
        (Value::Exact(_, left), Value::Exact(_, right)) if left != right => {
            program.new_exact_value(0)
        }
        _ => program.new_unknown_value(),
    }
}

/// Evaluate an instruction to determine the value of a register.
fn evaluate_instruction(
    program: &mut Program,
    instr: Instruction,
    left: Value,
    right: Value,
) -> Value {
    match instr {
        Instruction::Input(..) => unreachable!(),
        Instruction::Add(..) => evaluate_add(program, left, right),
        Instruction::Mul(..) => evaluate_mul(program, left, right),
        Instruction::Div(..) => evaluate_div(program, left, right),
        Instruction::Mod(..) => evaluate_mod(program, left, right),
        Instruction::Equal(..) => evaluate_eql(program, left, right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propagate_constants_removes_no_ops() {
        let instr: Vec<Instruction> = vec![
            Instruction::Add(Register(0), Operand::Literal(0)),
            Instruction::Div(Register(0), Operand::Literal(1)),
            Instruction::Mul(Register(0), Operand::Literal(0)),
            Instruction::Mod(Register(0), Operand::Literal(1)),
        ];

        assert_eq!(propagate_constants(instr), vec![],)
    }
}

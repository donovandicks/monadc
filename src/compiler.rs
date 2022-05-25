use crate::types::{Instruction, Operand};

/// Optimzation to remove no op binary instructions.
///
/// Removes div by 1, mul by 1, and add by 0 instructions.
///
/// These instructions can be removed because they have no effect
/// on the register supplied or the program as a whole.
pub fn remove_no_op_redundancies(instructions: Vec<Instruction>) -> Vec<Instruction> {
    instructions
        .into_iter()
        .filter(|instr| {
            !matches!(
                instr,
                Instruction::Div(_, Operand::Literal(1))
                    | Instruction::Add(_, Operand::Literal(0))
                    | Instruction::Mul(_, Operand::Literal(1))
            )
        })
        .collect()
}

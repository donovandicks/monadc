use crate::types::{Instruction, Operand};

/// Optimzation to remove division by 1 instructions.
///
/// This instruction can be removed because it has no affect
/// on the register supplied or the program as a whole.
pub fn remove_div_by_1(instructions: Vec<Instruction>) -> Vec<Instruction> {
    instructions
        .into_iter()
        .filter(|instr| !matches!(instr, Instruction::Div(_, Operand::Literal(1))))
        .collect()
}

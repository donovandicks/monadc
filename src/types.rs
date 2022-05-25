/// A register in a MONAD instruction.
///
/// MONAD supports only 4 variables:
///
/// - `w`: Register(0)
/// - `x`: Register(1)
/// - `y`: Register(2)
/// - `z`: Register(3)
#[derive(Clone, Debug, PartialEq)]
pub struct Register(pub usize);

/// The second operand of a MONAD instruction.
/// Can be a literal number of a register.
#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Literal(i64),
    Register(Register),
}

/// An instruction in the MONAD language.
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Input(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Equal(Register, Operand),
}

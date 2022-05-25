use std::fmt::Display;

use crate::utils::UniqueIdMaker;

/// An ID to track a particular `Value`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vid(pub usize);

impl Vid {
    #[inline]
    pub fn unique_id_maker() -> UniqueIdMaker<Vid> {
        UniqueIdMaker::starting_at(1)
    }
}

impl From<usize> for Vid {
    fn from(x: usize) -> Self {
        Self(x)
    }
}

/// A compiler value in the program
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Value {
    Exact(Vid, i64),
    Input(Vid, usize), // Represent the `i`th input to the program
    Unknown(Vid),
}

impl Value {
    pub fn vid(&self) -> Vid {
        match self {
            Value::Exact(vid, _) | Value::Input(vid, _) | Value::Unknown(vid) => *vid,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Self::Exact(_, left), &Self::Exact(_, right)) => {
                // Check if two known constants are equal
                left == right
            }
            (&left, &right) => {
                // Check if two inputs or unknowns have the same `Vid`
                left.vid() == right.vid()
            }
        }
    }
}

/// A register in a MONAD instruction.
///
/// MONAD supports only 4 variables:
///
/// - `w`: Register(0)
/// - `x`: Register(1)
/// - `y`: Register(2)
/// - `z`: Register(3)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Register(pub usize);

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self.0 {
            0 => 'w',
            1 => 'x',
            2 => 'y',
            3 => 'z',
            _ => unreachable!("{:?}", self),
        };

        write!(f, "{}", letter)
    }
}

/// The second operand of a MONAD instruction.
/// Can be a literal number of a register.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    Literal(i64),
    Register(Register),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Literal(l) => write!(f, "{}", l),
            Operand::Register(r) => write!(f, "{}", *r),
        }
    }
}

/// An instruction in the MONAD language.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    Input(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Equal(Register, Operand),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Input(r) => write!(f, "inp {}", *r),
            Instruction::Add(r, o) => write!(f, "add {} {}", *r, *o),
            Instruction::Mul(r, o) => write!(f, "mul {} {}", *r, *o),
            Instruction::Div(r, o) => write!(f, "div {} {}", *r, *o),
            Instruction::Mod(r, o) => write!(f, "mod {} {}", *r, *o),
            Instruction::Equal(r, o) => write!(f, "eql {} {}", *r, *o),
        }
    }
}

impl Instruction {
    /// Retrieves the destination register of the instruction.
    #[inline]
    pub fn destination(&self) -> usize {
        match self {
            Instruction::Input(r)
            | Instruction::Add(r, _)
            | Instruction::Mul(r, _)
            | Instruction::Div(r, _)
            | Instruction::Mod(r, _)
            | Instruction::Equal(r, _) => r,
        }
        .0 // Retrieve the first field from the Register (the index)
    }

    /// Retrieves the operand of the instruction.
    #[inline]
    pub fn operand(&self) -> Option<Operand> {
        match self {
            Instruction::Input(_) => None,
            Instruction::Add(_, o)
            | Instruction::Mul(_, o)
            | Instruction::Div(_, o)
            | Instruction::Mod(_, o)
            | Instruction::Equal(_, o) => Some(*o),
        }
    }
}

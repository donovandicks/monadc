use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, one_of, space1},
    combinator::{map, map_res, opt, recognize},
    multi::many1,
    sequence::tuple,
    IResult,
};

use crate::types::{Instruction, Operand, Register};

/// Parse a register
fn register(input: &str) -> IResult<&str, Register> {
    let (remainder, matched_char) = one_of("wxyz")(input)?;
    let register_id = match matched_char {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => unreachable!("{}", matched_char),
    };

    Ok((remainder, Register(register_id)))
}

/// Parse a signed int
fn text_signed_int(input: &str) -> IResult<&str, i64> {
    map_res(recognize(tuple((opt(char('-')), digit1))), |value: &str| {
        value.parse()
    })(input)
}

/// Parse an operand, which is either a signed int or a register
fn operand(input: &str) -> IResult<&str, Operand> {
    if let Ok((remainder, register)) = register(input) {
        Ok((remainder, Operand::Register(register)))
    } else {
        map(text_signed_int, Operand::Literal)(input)
    }
}

/// Parse a full input instruction.
///
/// # Example
/// `inp x`
fn input_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((tag("inp"), space1, register, opt(line_ending))),
        |(_, _, reg, _)| Instruction::Input(reg),
    )(input)
}

/// Parse a full binary instruction.
///
/// A binary instruction is any instruction that takes two inputs.
/// The first input __must__ be a register, and the second input
/// may be _either_ a register _or_ a literal number.
///
/// # Example
/// `add x 1`
fn binary_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            alt((tag("add"), tag("mul"), tag("div"), tag("mod"), tag("eql"))),
            space1,
            register,
            space1,
            operand,
            opt(line_ending),
        )),
        |(instr, _, reg, _, val, _)| match instr {
            "add" => Instruction::Add(reg, val),
            "mul" => Instruction::Mul(reg, val),
            "div" => Instruction::Div(reg, val),
            "mod" => Instruction::Mod(reg, val),
            "eql" => Instruction::Equal(reg, val),
            _ => unreachable!("{}", instr),
        },
    )(input)
}

/// Parse any full instruction.
///
/// A full instruction is either an input instruction or a binary
/// instruction.
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((input_instruction, binary_instruction))(input)
}

/// Parse a full MONAD program.
///
/// A full program is a collection of binary or input instructions.
///
/// # Example
/// ```
/// let input = "inp x
/// mul x 2";
/// ```
pub fn parse_program(input: &str) -> Vec<Instruction> {
    let (remainder, program) = many1(instruction)(input).unwrap();
    assert!(remainder.is_empty());
    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program_valid_input_instr() {
        assert_eq!(
            parse_program(
                "inp w
add x 1
add y -1
mul x y
div y x
mod z w
eql w z"
            ),
            vec![
                Instruction::Input(Register(0)),
                Instruction::Add(Register(1), Operand::Literal(1)),
                Instruction::Add(Register(2), Operand::Literal(-1)),
                Instruction::Mul(Register(1), Operand::Register(Register(2))),
                Instruction::Div(Register(2), Operand::Register(Register(1))),
                Instruction::Mod(Register(3), Operand::Register(Register(0))),
                Instruction::Equal(Register(0), Operand::Register(Register(3))),
            ]
        )
    }
}

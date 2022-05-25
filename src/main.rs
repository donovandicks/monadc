use std::{env, fs};

use monadc::{compiler::remove_no_op_redundancies, parser::parse_program, types::Instruction};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let content = fs::read_to_string(args.pop().expect("input file path")).unwrap();

    let program = parse_program(content.as_str());
    analyze(program)
}

fn get_percent_difference(original_len: usize, optimized_len: usize) -> f64 {
    let orig_float = f64::from(original_len as i32);
    let optim_float = f64::from(optimized_len as i32);

    ((orig_float / optim_float) - 1.0) * 100.0
}

fn analyze(program: Vec<Instruction>) {
    let original = program.clone();
    let optimized = remove_no_op_redundancies(program);

    let original_len = original.len();
    let optimized_len = optimized.len();

    println!(
        "Original vs. Optimized Length:\t{} vs {}",
        original_len, optimized_len
    );

    println!(
        "Optimization is {:.2}% more efficient.",
        get_percent_difference(original_len, optimized_len)
    );
}

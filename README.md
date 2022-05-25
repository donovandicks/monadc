# Monadc

A compiler for Advent of Code Day 24

## Problem

See full text [here](https://adventofcode.com/2021/day/24)

> The ALU is a four-dimensional processing unit: it has integer variables w, x, y,
and z. These variables all start with the value 0. The ALU also supports six
instructions:

    inp a - Read an input value and write it to variable a.
    add a b - Add the value of a to the value of b, then store the result in variable a.
    mul a b - Multiply the value of a by the value of b, then store the result in variable a.
    div a b - Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
    mod a b - Divide the value of a by the value of b, then store the remainder in variable a. (This is also called the modulo operation.)
    eql a b - If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.

## References

[Compiler Adventures](https://medium.com/@predrag.gruevski/compiler-adventures-part-1-no-op-instructions-c084358c7864)
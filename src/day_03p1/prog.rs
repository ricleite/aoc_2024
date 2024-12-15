/*
--- Day 3: Mull It Over ---

"Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.

The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"

The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!

It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.

However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.

For example, consider the following section of corrupted memory:

xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).

Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications?
 */

use std::fs::read_to_string;

fn main() {
    let mut sum = 0;
    for line in read_to_string("input").unwrap().lines() {
        // should get an award for the absolute dumbest parsing ever
        // complete abuse of split() as a primitive
        for mul in line.split("mul(") {
            let v: Vec<&str> = mul.split(",").collect();
            if v.len() < 2 {
                continue;
            }

            let n1 = v[0];
            if !n1.chars().all(|c| char::is_numeric(c)) {
                continue;
            }

            let v2: Vec<&str> = v[1].split(")").collect();
            if v2.len() == 0 {
                continue;
            }

            let n2 = v2[0];
            if !n2.chars().all(|c| char::is_numeric(c)) {
                continue;
            }

            sum += n1.parse::<u64>().unwrap() * n2.parse::<u64>().unwrap();
        }
    }

    println!("{}", sum);
}
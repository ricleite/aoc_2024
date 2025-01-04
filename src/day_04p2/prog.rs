/*
--- Day 4: Ceres Search ---

"Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:

..X...
.SAMX.
.A..A.
XMAS.S
.X....

The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX

In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX

Take a look at the little Elf's word search. How many times does XMAS appear?

--- Part Two ---

The Elf looks quizzically at you. Did you misunderstand the assignment?

Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:

M.S
.A.
M.S

Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.

Here's the same example from before, but this time all of the X-MASes have been kept instead:

.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........

In this example, an X-MAS appears 9 times.

Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear?
*/

use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input: String = read_to_string(filename).unwrap();
 
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    // https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    // also disgusting
    let mut grid_raw = vec!['\0'; height * width];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let grid = grid_base.as_mut_slice();
    for (i, line) in lines.into_iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    // patterns to search for
    let patterns = vec![
        ('A', 'M', 'S'),
        ('A', 'S', 'M')
    ];
    // and layout to use for searches
    // NOTE: need to declare type because deduction fails otherwise..
    let layouts: Vec<((i64,i64),(i64,i64))> = vec![
        ((1, 1), (-1, -1)),
        ((1, -1), (-1, 1)),
    ];

    let mut matches = 0;
    for x in 1..height-1 {
        for y in 1..width-1 {
            let mut local_match = 0;
            for (c1, c2, c3) in &patterns {
                if grid[x][y] != *c1 {
                    continue;
                }

                for ((x2, y2), (x3, y3)) in &layouts {
                    // these casts are disgusting
                    // but how else to work with the negative offset?
                    let xi = x as i64;
                    let yi = y as i64;
                    // this is terrible
                    if grid[(xi+x2) as usize][(yi+y2) as usize] == *c2
                        && grid[(xi+x3) as usize][(yi+y3) as usize] == *c3 {
                        local_match += 1;
                    }
                }
            }

            if local_match >= 2 {
                matches += 1;
            }
        } 
    }

    println!("{}", matches);
}
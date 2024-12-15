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
*/

use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();
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
        ('X', 'M', 'A', 'S'),
        ('S', 'A', 'M', 'X')
    ];
    // and layout to use for searches
    // NOTE: need to declare type because deduction fails otherwise..
    let layouts: Vec<((i64,i64),(i64,i64),(i64,i64))> = vec![
        ((0, 1), (0, 2), (0, 3)),
        ((1, 0), (2, 0), (3, 0)),
        ((1, 1), (2, 2), (3, 3)),
        ((1, -1), (2, -2), (3, -3))
    ];

    let mut matches = 0;
    for (c1, c2, c3, c4) in patterns {
        for x in 0..height {
            for y in 0..width {
                if grid[x][y] != c1 {
                    continue;
                }

                for ((x2, y2), (x3, y3), (x4, y4)) in &layouts {
                    // these casts are disgusting
                    // but how else to work with the negative offset?
                    let xi = x as i64;
                    let yi = y as i64;
                    let xr = xi + x4;
                    let yr = yi + y4;
                    if xr < 0 || xr >= width as i64 || yr < 0 || yr >= height as i64 {
                        continue;
                    }

                    // this is terrible
                    if grid[(xi+x2) as usize][(yi+y2) as usize] == c2
                        && grid[(xi+x3) as usize][(yi+y3) as usize] == c3
                        && grid[(xi+x4) as usize][(yi+y4) as usize] == c4 {
                        matches += 1;
                    }
                }
            } 
        }
    }

    println!("{}", matches);
}
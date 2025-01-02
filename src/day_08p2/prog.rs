/*
--- Day 8: Resonant Collinearity ---

You find yourselves on the roof of a top-secret Easter Bunny installation.

While The Historians do their thing, you take a look at the familiar huge antenna. Much to your surprise, it seems to have been reconfigured to emit a signal that makes people 0.1% more likely to buy Easter Bunny brand Imitation Mediocre Chocolate as a Christmas gift! Unthinkable!

Scanning across the city, you find that there are actually many such antennas. Each antenna is tuned to a specific frequency indicated by a single lowercase letter, uppercase letter, or digit. You create a map (your puzzle input) of these antennas. For example:

............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............

The signal only applies its nefarious effect at specific antinodes based on the resonant frequencies of the antennas. In particular, an antinode occurs at any point that is perfectly in line with two antennas of the same frequency - but only when one of the antennas is twice as far away as the other. This means that for any pair of antennas with the same frequency, there are two antinodes, one on either side of them.

So, for these two antennas with frequency a, they create the two antinodes marked with #:

..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........

Adding a third antenna with the same frequency creates several more antinodes. It would ideally add four antinodes, but two are off the right side of the map, so instead it adds only two:

..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........

Antennas with different frequencies don't create antinodes; A and a count as different frequencies. However, antinodes can occur at locations that contain antennas. In this diagram, the lone antenna with frequency capital A creates no antinodes but has a lowercase-a-frequency antinode at its location:

..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
..........

The first example has antennas with two different frequencies, so the antinodes they create look like this, plus an antinode overlapping the topmost A-frequency antenna:

......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.

Because the topmost A-frequency antenna overlaps with a 0-frequency antinode, there are 14 total unique locations that contain an antinode within the bounds of the map.

Calculate the impact of the signal. How many unique locations within the bounds of the map contain an antinode?

--- Part Two ---

Watching over your shoulder as you work, one of The Historians asks if you took the effects of resonant harmonics into your calculations.

Whoops!

After updating your model, it turns out that an antinode occurs at any grid position exactly in line with at least two antennas of the same frequency, regardless of distance. This means that some of the new antinodes will occur at the position of each antenna (unless that antenna is the only one of its frequency).

So, these three T-frequency antennas now create many antinodes:

T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........

In fact, the three T-frequency antennas are all exactly in line with two antennas, so they are all also antinodes! This brings the total number of antinodes in the above example to 9.

The original example now has 34 antinodes, including the antinodes that appear on every antenna:

##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##

Calculate the impact of the signal using this updated model. How many unique locations within the bounds of the map contain an antinode?
*/

use std::env;
use std::fs::read_to_string;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

// indexes are small, so the (absolutely terrible!) efficiency here does not matter
fn gcd(a: usize, b: usize) -> usize {
    let mut res = a.min(b);
    while res > 0 {
        if a % res == 0 && b % res == 0 {
            return res;
        }

        res -= 1;
    }

    return res;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = read_to_string(filename).unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let max_x = map.len();
    let max_y = map.first().unwrap().len();

    let mut signals: BTreeMap<char, Vec<(usize, usize)>> = BTreeMap::new();
    let mut antinodes: BTreeSet<(usize, usize)> = BTreeSet::new();
    for x in 0..max_x {
        for y in 0..max_y {
            if map[x][y] == '.' {
                continue;
            }

            let c = map[x][y];
            match signals.get(&c) {
                Some(v) => {
                    for (ex, ey) in v {
                        let (mut dx, mut dy) = (*ex as i64 - x as i64, *ey as i64 - y as i64);
                        let gcd = gcd(dx.abs() as usize, dy.abs() as usize);
                        if gcd > 0 {
                            dx = dx / gcd as i64;
                            dy = dy / gcd as i64;
                        }

                        for i in 0..i64::MAX {
                            let new_x = (x as i64 + dx * i) as usize;
                            let new_y = (y as i64 + dy * i) as usize;
                            if new_x < max_x && new_y < max_y {
                                antinodes.insert((new_x, new_y));
                            } else {
                                break;
                            }
                        }

                        for i in 0..i64::MAX {
                            let new_x = (x as i64 - dx * i) as usize;
                            let new_y = (y as i64 - dy * i) as usize;
                            if new_x < max_x && new_y < max_y {
                                antinodes.insert((new_x, new_y));
                            } else {
                                break;
                            }
                        }
                    }
                },
                _ => ()
            }

            signals.entry(c).or_insert(vec![]).push((x, y));
        }
    }

    println!("{}", antinodes.len());
}

/*
--- Day 20: Race Condition ---

The Historians are quite pixelated again. This time, a massive, black building looms over you - you're right outside the CPU!

While The Historians get to work, a nearby program sees that you're idle and challenges you to a race. Apparently, you've arrived just in time for the frequently-held race condition festival!

The race takes place on a particularly long and twisting code path; programs compete to see who can finish in the fewest picoseconds. The winner even gets their very own mutex!

They hand you a map of the racetrack (your puzzle input). For example:

###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

The map consists of track (.) - including the start (S) and end (E) positions (both of which also count as track) - and walls (#).

When a program runs through the racetrack, it starts at the start position. Then, it is allowed to move up, down, left, or right; each such move takes 1 picosecond. The goal is to reach the end position as quickly as possible. In this example racetrack, the fastest time is 84 picoseconds.

Because there is only a single path from the start to the end and the programs all go the same speed, the races used to be pretty boring. To make things more interesting, they introduced a new rule to the races: programs are allowed to cheat.

The rules for cheating are very strict. Exactly once during a race, a program may disable collision for up to 2 picoseconds. This allows the program to pass through walls as if they were regular track. At the end of the cheat, the program must be back on normal track again; otherwise, it will receive a segmentation fault and get disqualified.

So, a program could complete the course in 72 picoseconds (saving 12 picoseconds) by cheating for the two moves marked 1 and 2:

###############
#...#...12....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

Or, a program could complete the course in 64 picoseconds (saving 20 picoseconds) by cheating for the two moves marked 1 and 2:

###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...12..#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

This cheat saves 38 picoseconds:

###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.####1##.###
#...###.2.#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

This cheat saves 64 picoseconds and takes the program directly to the end:

###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..21...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

Each cheat has a distinct start position (the position where the cheat is activated, just before the first move that is allowed to go through walls) and end position; cheats are uniquely identified by their start position and end position.

In this example, the total number of cheats (grouped by the amount of time they save) are as follows:

    There are 14 cheats that save 2 picoseconds.
    There are 14 cheats that save 4 picoseconds.
    There are 2 cheats that save 6 picoseconds.
    There are 4 cheats that save 8 picoseconds.
    There are 2 cheats that save 10 picoseconds.
    There are 3 cheats that save 12 picoseconds.
    There is one cheat that saves 20 picoseconds.
    There is one cheat that saves 36 picoseconds.
    There is one cheat that saves 38 picoseconds.
    There is one cheat that saves 40 picoseconds.
    There is one cheat that saves 64 picoseconds.

You aren't sure what the conditions of the racetrack will be like, so to give yourself as many options as possible, you'll need a list of the best cheats. How many cheats would save you at least 100 picoseconds?
*/

use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = read_to_string(filename).unwrap();

    let mut itr = input.lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in itr.by_ref().take_while(|s| !s.is_empty()) {
        let v: Vec<char> = line.chars().collect();
        map.push(v);
    }

    let threshold = itr.next().unwrap().parse::<u32>().unwrap();

    let max_x = map.len();
    let max_y = map[0].len();

    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            match map[x][y] {
                'S' => (start_x, start_y) = (x, y),
                'E' => (end_x, end_y) = (x, y),
                _ => ()
            }
        }
    }

    // idea: compute weights first for fastest path
    // then we do another step to compute the cheats
    let mut weights: Vec<Vec<u32>> = map.iter()
        .map(|v| vec![u32::MAX; v.len()]).collect();

    weights[end_x][end_y] = 0;

    let mut curr: Vec<(u32, usize, usize)> = vec![(0, end_x, end_y)];
    while !curr.is_empty() {
        let mut next: Vec<(u32, usize, usize)> = Vec::new();
        for (dist, x, y) in &curr {
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                // no bounds check necessary, because there is a wall surrounding track
                let new_dist = dist + 1;
                let new_x = (*x as i32 + dx) as usize;
                let new_y = (*y as i32 + dy) as usize;

                if map[new_x][new_y] == '#' || weights[new_x][new_y] <= new_dist {
                    continue;
                }

                weights[new_x][new_y] = new_dist;
                next.push((new_dist, new_x, new_y));
            }
        }

        curr = next;
    }

    let mut cheat_count = 0;

    // now, for the calculation with the cheats, quite simple
    // 1) go through the entire map
    // 2) for every position, using the weights, check how much a "bypass" would save
    for x in 0..max_x {
        for y in 0..max_y {
            if weights[x][y] == u32::MAX {
                continue;
            }

            for (dx, dy) in [(0, 1), (0, 2), (1, 0), (2, 0), (0, -1), (0, -2 ), (-1, 0), (-2, 0)] {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;
                let delta: u32 = (dx.abs() + dy.abs()) as u32;
                if new_x >= max_x || new_y >= max_y || weights[new_x][new_y] == u32::MAX {
                    continue;
                }

                // we have a cheat!
                if weights[x][y] > weights[new_x][new_y] + delta {
                    let diff = weights[x][y] - (weights[new_x][new_y] + delta);
                    if diff >= threshold {
                        cheat_count += 1;
                    }
                }
            }
        }
    }

    println!("{}, {}", weights[start_x][start_y], cheat_count);
}
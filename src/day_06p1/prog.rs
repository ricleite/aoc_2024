/*
--- Day 6: Guard Gallivant ---

The Historians use their fancy device again, this time to whisk you all away to the North Pole prototype suit manufacturing lab... in the year 1518! It turns out that having direct access to history is very convenient for a group of historians.

You still have to be careful of time paradoxes, and so it will be important to avoid anyone from 1518 while The Historians search for the Chief. Unfortunately, a single guard is patrolling this part of the lab.

Maybe you can work out where the guard will go ahead of time so that The Historians can search safely?

You start by making a map (your puzzle input) of the situation. For example:

....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...

The map shows the current position of the guard with ^ (to indicate the guard is currently facing up from the perspective of the map). Any obstructions - crates, desks, alchemical reactors, etc. - are shown as #.

Lab guards in 1518 follow a very strict patrol protocol which involves repeatedly following these steps:

    If there is something directly in front of you, turn right 90 degrees.
    Otherwise, take a step forward.

Following the above protocol, the guard moves up several times until she reaches an obstacle (in this case, a pile of failed suit prototypes):

....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...

Because there is now an obstacle in front of the guard, she turns right before continuing straight in her new facing direction:

....#.....
........>#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...

Reaching another obstacle (a spool of several very long polymers), she turns right again and continues downward:

....#.....
.........#
..........
..#.......
.......#..
..........
.#......v.
........#.
#.........
......#...

This process continues for a while, but the guard eventually leaves the mapped area (after walking past a tank of universal solvent):

....#.....
.........#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#v..

By predicting the guard's route, you can determine which specific positions in the lab will be in the patrol path. Including the guard's starting position, the positions visited by the guard before leaving the area are marked with an X:

....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..

In this example, the guard will visit 41 distinct positions on your map.

Predict the path of the guard. How many distinct positions will the guard visit before leaving the mapped area?
*/

use std::env;
use std::fs::read_to_string;
use std::collections::BTreeSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = read_to_string(filename).unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let max_x = map.len();
    let max_y = map.first().unwrap().len();

    let (mut curr_x, mut curr_y) = (0, 0);
    let (mut dx, mut dy) = (-1, 0);
    for x in 0..max_x {
        for y in 0..max_y {
            if map[x][y] == '^' {
                (curr_x, curr_y) = (x, y);
                (dx, dy) = (-1, 0);
            }
        }
    }

    let mut pos: BTreeSet<(usize, usize)> = BTreeSet::new();

    loop {
        let (new_x, new_y) = ((curr_x as i32 + dx) as usize, (curr_y as i32 + dy) as usize);
        if new_x >= max_x || new_y >= max_y {
            break;
        }

        // oupsies cannot go there, turn 90o, then go at it again
        if map[new_x][new_y] == '#' {
            (dx, dy) = match (dx, dy) {
                (1, 0) => (0, -1),
                (0, 1) => (1, 0),
                (-1, 0) => (0, 1),
                (0, -1) => (-1, 0),
                _ => panic!()
            };

            continue;
        }

        pos.insert((new_x, new_y));
        (curr_x, curr_y) = (new_x, new_y);
    }

    println!("{}", pos.len());
}
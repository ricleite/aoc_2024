/*
--- Day 16: Reindeer Maze ---

It's time again for the Reindeer Olympics! This year, the big event is the Reindeer Maze, where the Reindeer compete for the lowest score.

You and The Historians arrive to search for the Chief right as the event is about to start. It wouldn't hurt to watch a little, right?

The Reindeer start on the Start Tile (marked S) facing East and need to reach the End Tile (marked E). They can move forward one tile at a time (increasing their score by 1 point), but never into a wall (#). They can also rotate clockwise or counterclockwise 90 degrees at a time (increasing their score by 1000 points).

To figure out the best place to sit, you start by grabbing a map (your puzzle input) from a nearby kiosk. For example:

###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############

There are many paths through this maze, but taking any of the best paths would incur a score of only 7036. This can be achieved by taking a total of 36 steps forward and turning 90 degrees a total of 7 times:


###############
#.......#....E#
#.#.###.#.###^#
#.....#.#...#^#
#.###.#####.#^#
#.#.#.......#^#
#.#.#####.###^#
#..>>>>>>>>v#^#
###^#.#####v#^#
#>>^#.....#v#^#
#^#.#.###.#v#^#
#^....#...#v#^#
#^###.#.#.#v#^#
#S..#.....#>>^#
###############

Here's a second example:

#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################

In this maze, the best paths cost 11048 points; following one such path would look like this:

#################
#...#...#...#..E#
#.#.#.#.#.#.#.#^#
#.#.#.#...#...#^#
#.#.#.#.###.#.#^#
#>>v#.#.#.....#^#
#^#v#.#.#.#####^#
#^#v..#.#.#>>>>^#
#^#v#####.#^###.#
#^#v#..>>>>^#...#
#^#v###^#####.###
#^#v#>>^#.....#.#
#^#v#^#####.###.#
#^#v#^........#.#
#^#v#^#########.#
#S#>>^..........#
#################

Note that the path shown above includes one 90 degree turn as the very first move, rotating the Reindeer from facing East to facing North.

Analyze your map carefully. What is the lowest score a Reindeer could possibly get?
*/

use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = read_to_string(filename).unwrap();

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let v: Vec<char> = line.chars().collect();
        map.push(v);
    }

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

    let mut weights: Vec<Vec<u32>> = Vec::new();
    for v in &map {
        weights.push(vec![u32::MAX; v.len()]);
    }

    let mut curr: Vec<(usize, usize, u32, i32, i32)> = vec![
        (start_x, start_y, 0, 0, 1),
    ];
    let mut next: Vec<(usize, usize, u32, i32, i32)> = Vec::new();

    while curr.len() > 0 {
        for (x, y, score, dir_x, dir_y) in &curr {
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                // cannot go backwards, only forward/sideways
                if dir_x + dx == 0 && dir_y + dy == 0 {
                    continue;
                }

                let plus_score = if *dir_x == dx && *dir_y == dy {
                    1
                } else {
                    1001
                };

                let new_x = (*x as i32 + dx) as usize;
                let new_y = (*y as i32 + dy) as usize;
                let new_score = score + plus_score;

                match map[new_x][new_y] {
                    '#' => (),
                    _ => {
                        if weights[new_x][new_y] <= new_score {
                            continue;
                        }

                        weights[new_x][new_y] = new_score;
                        next.push((new_x, new_y, new_score, dx, dy));
                    }
                }
            }
        }

        curr = next;
        next = Vec::new();
    }

    println!("{}", weights[end_x][end_y]);
}
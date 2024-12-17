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

--- Part Two ---

Now that you know what the best paths look like, you can figure out the best spot to sit.

Every non-wall tile (S, ., or E) is equipped with places to sit along the edges of the tile. While determining which of these tiles would be the best spot to sit depends on a whole bunch of factors (how comfortable the seats are, how far away the bathrooms are, whether there's a pillar blocking your view, etc.), the most important factor is whether the tile is on one of the best paths through the maze. If you sit somewhere else, you'd miss all the action!

So, you'll need to determine which tiles are part of any best path through the maze, including the S and E tiles.

In the first example, there are 45 tiles (marked O) that are part of at least one of the various best paths through the maze:

###############
#.......#....O#
#.#.###.#.###O#
#.....#.#...#O#
#.###.#####.#O#
#.#.#.......#O#
#.#.#####.###O#
#..OOOOOOOOO#O#
###O#O#####O#O#
#OOO#O....#O#O#
#O#O#O###.#O#O#
#OOOOO#...#O#O#
#O###.#.#.#O#O#
#O..#.....#OOO#
###############

In the second example, there are 64 tiles that are part of at least one of the best paths:

#################
#...#...#...#..O#
#.#.#.#.#.#.#.#O#
#.#.#.#...#...#O#
#.#.#.#.###.#.#O#
#OOO#.#.#.....#O#
#O#O#.#.#.#####O#
#O#O..#.#.#OOOOO#
#O#O#####.#O###O#
#O#O#..OOOOO#OOO#
#O#O###O#####O###
#O#O#OOO#..OOO#.#
#O#O#O#####O###.#
#O#O#OOOOOOO..#.#
#O#O#O#########.#
#O#OOO..........#
#################

Analyze your map further. How many tiles are part of at least one of the best paths through the maze?
*/

use std::env;
use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::u32;

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

    let mut best: HashSet<(usize, usize)> = HashSet::from([(end_x, end_y)]);
    let mut heap: BinaryHeap<Reverse<(u32, usize, usize, i32, i32, Vec<(usize, usize)>)>> = BinaryHeap::new();
    let mut score_at_end = u32::MAX;
    let mut weights: Vec<Vec<u32>> = Vec::new();
    for v in &map {
        weights.push(vec![u32::MAX; v.len()]);
    }

    heap.push(Reverse((0, start_x, start_y, 0, 1, vec![(start_x, start_y)])));
    while heap.len() > 0 {
        let Reverse((score, x, y, dir_x, dir_y, v)) = heap.pop().unwrap();

        if score > score_at_end {
            break;
        }

        // how does this work at all? it absolutely *SHOULD NOT*!
        if best.contains(&(x, y)) {
            for (p_x, p_y) in v {
                best.insert((p_x, p_y));
            }

            if (x, y) == (end_x, end_y) {
                score_at_end = score;
            }

            continue;
        }

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            // cannot go backwards, only forward/sideways
            if dir_x + dx == 0 && dir_y + dy == 0 {
                continue;
            }

            let plus_score = if dir_x == dx && dir_y == dy {
                1
            } else {
                1001
            };

            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            let new_score = score + plus_score;

            match map[new_x][new_y] {
                '#' => (),
                _ => {
                    // TERRIBLE!
                    if weights[new_x][new_y] < new_score - u32::min(new_score, 1001) {
                        continue;
                    }

                    weights[new_x][new_y] = new_score;
                    let mut v2 = v.clone();
                    v2.push((new_x, new_y));
                    heap.push(Reverse((new_score, new_x, new_y, dx, dy, v2)));
                }
            }
        }
    }

    let debug = false;
    if debug {
        for x in 0..max_x {
            for y in 0..max_y {
                if best.contains(&(x, y)) {
                    print!("O");
                } else {
                    print!("{}", map[x][y]);
                }
            }
            println!("");
        }
    }

    println!("{}", best.len());
}
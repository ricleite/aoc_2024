
/*
--- Day 15: Warehouse Woes ---

You appear back inside your own mini submarine! Each Historian drives their mini submarine in a different direction; maybe the Chief has his own submarine down here somewhere as well?

You look up to see a vast school of lanternfish swimming past you. On closer inspection, they seem quite anxious, so you drive your mini submarine over to see if you can help.

Because lanternfish populations grow rapidly, they need a lot of food, and that food needs to be stored somewhere. That's why these lanternfish have built elaborate warehouse complexes operated by robots!

These lanternfish seem so anxious because they have lost control of the robot that operates one of their most important warehouses! It is currently running amok, pushing around boxes in the warehouse with no regard for lanternfish logistics or lanternfish inventory management strategies.

Right now, none of the lanternfish are brave enough to swim up to an unpredictable robot so they could shut it off. However, if you could anticipate the robot's movements, maybe they could find a safe option.

The lanternfish already have a map of the warehouse and a list of movements the robot will attempt to make (your puzzle input). The problem is that the movements will sometimes fail as boxes are shifted around, making the actual movements of the robot difficult to predict.

For example:

##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^

As the robot (@) attempts to move, if there are any boxes (O) in the way, the robot will also attempt to push those boxes. However, if this action would cause the robot or a box to move into a wall (#), nothing moves instead, including the robot. The initial positions of these are shown on the map at the top of the document the lanternfish gave you.

The rest of the document describes the moves (^ for up, v for down, < for left, > for right) that the robot will attempt to make, in order. (The moves form a single giant sequence; they are broken into multiple lines just to make copy-pasting easier. Newlines within the move sequence should be ignored.)

Here is a smaller example to get started:

########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<

Were the robot to attempt the given sequence of moves, it would push around the boxes as follows:

Initial state:
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move <:
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move ^:
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move ^:
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move v:
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.@...#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##.....#
#..@O..#
#.#.O..#
#...O..#
#...O..#
########

Move >:
########
#....OO#
##.....#
#...@O.#
#.#.O..#
#...O..#
#...O..#
########

Move >:
########
#....OO#
##.....#
#....@O#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##.....#
#.....O#
#.#.O@.#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########

The larger example has many more moves; after the robot has finished those moves, the warehouse would look like this:

##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########

The lanternfish use their own custom Goods Positioning System (GPS for short) to track the locations of the boxes. The GPS coordinate of a box is equal to 100 times its distance from the top edge of the map plus its distance from the left edge of the map. (This process does not stop at wall tiles; measure all the way to the edges of the map.)

So, the box shown below has a distance of 1 from the top edge of the map and 4 from the left edge of the map, resulting in a GPS coordinate of 100 * 1 + 4 = 104.

#######
#...O..
#......

The lanternfish would like to know the sum of all boxes' GPS coordinates after the robot finishes moving. In the larger example, the sum of all boxes' GPS coordinates is 10092. In the smaller example, the sum is 2028.

Predict the motion of the robot and boxes in the warehouse. After the robot is finished moving, what is the sum of all boxes' GPS coordinates?
*/

use std::env; // misleading name, I want it to read argv
use std::fs::read_to_string;

// advance a x/y pair by dx/dy
// assumes no underflow/overflow
fn advance(x: usize, y: usize, dx: i32, dy: i32) -> (usize, usize) {
    return ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
}

fn print(map: &Vec<Vec<char>>) {
    let max_x = map.len();
    let max_y = map.first().unwrap().len();
    for x in 0..max_x {
        for y in 0..max_y {
            print!("{}", map[x][y]);
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let input = read_to_string(filename).unwrap();

    // loaded environment
    let mut map: Vec<Vec<char>> = Vec::new(); // map
    let mut instructions: Vec<char> = Vec::new(); // instructions

    let mut itr = input.lines().into_iter();
    // is there a better way to consume an iterator partially? probably..
    loop {
        let s = itr.next().unwrap();
        if s.is_empty() {
            break;
        }

        let v: Vec<char> = s.chars().collect();
        map.push(v);
    }

    loop {
        let r = itr.next();
        if r.is_none() {
            break;
        }

        let mut v: Vec<char> = r.unwrap().chars().collect();
        instructions.append(&mut v);
    }

    let max_x = map.len();
    let max_y = map.first().unwrap().len();

    // find start pos
    let mut curr_x = 0;
    let mut curr_y = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            if map[x][y] == '@' {
                curr_x = x;
                curr_y = y;
            }
        }
    }

    let debug = false;

    for instr in instructions {
        let (dx, dy) = match &instr {
            'v' => (1, 0),
            '^' => (-1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            _ => panic!()
        };

        let mut max_steps = 1;
        let mut steps = 0;
        let mut boxes = 0;
        let mut itr_x = curr_x;
        let mut itr_y = curr_y;
        loop {
            steps += 1;
            (itr_x, itr_y) = advance(itr_x, itr_y, dx, dy);
            match map[itr_x][itr_y] {
                'O' => boxes += 1,
                '.' => {
                    // do only `max_steps`
                    if max_steps == 0 {
                        break;
                    } else {
                        max_steps -= 1;
                    }
                },
                '#' => {
                    // hit wall, stop immediately
                    break;
                },
                _ => panic!()
            }
            map[itr_x][itr_y] = '.';
        }

        // update map with boxes
        for _ in 0..boxes {
            // backtrack position
            (itr_x, itr_y) = advance(itr_x, itr_y, -dx, -dy);
            map[itr_x][itr_y] = 'O';
        }
        // update map with new position
        (itr_x, itr_y) = advance(itr_x, itr_y, -dx, -dy);
        // then, update position
        map[curr_x][curr_y] = '.';
        (curr_x, curr_y) = (itr_x, itr_y);
        map[curr_x][curr_y] = '@';


        if debug {
            dbg!(instr, steps, boxes);
            print(&map);
        }
    }

    let mut res = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            if map[x][y] == 'O' {
                res += 100 * x + y;
            }
        }
    }

    println!("{}", res);
}
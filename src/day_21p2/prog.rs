/*
--- Day 21: Keypad Conundrum ---

As you teleport onto Santa's Reindeer-class starship, The Historians begin to panic: someone from their search party is missing. A quick life-form scan by the ship's computer reveals that when the missing Historian teleported, he arrived in another part of the ship.

The door to that area is locked, but the computer can't open it; it can only be opened by physically typing the door codes (your puzzle input) on the numeric keypad on the door.

The numeric keypad has four rows of buttons: 789, 456, 123, and finally an empty gap followed by 0A. Visually, they are arranged like this:

+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

Unfortunately, the area outside the door is currently depressurized and nobody can go near the door. A robot needs to be sent instead.

The robot has no problem navigating the ship and finding the numeric keypad, but it's not designed for button pushing: it can't be told to push a specific button directly. Instead, it has a robotic arm that can be controlled remotely via a directional keypad.

The directional keypad has two rows of buttons: a gap / ^ (up) / A (activate) on the first row and < (left) / v (down) / > (right) on the second row. Visually, they are arranged like this:

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

When the robot arrives at the numeric keypad, its robotic arm is pointed at the A button in the bottom right corner. After that, this directional keypad remote control must be used to maneuver the robotic arm: the up / down / left / right buttons cause it to move its arm one button in that direction, and the A button causes the robot to briefly move forward, pressing the button being aimed at by the robotic arm.

For example, to make the robot type 029A on the numeric keypad, one sequence of inputs on the directional keypad you could use is:

    < to move the arm from A (its initial position) to 0.
    A to push the 0 button.
    ^A to move the arm to the 2 button and push it.
    >^^A to move the arm to the 9 button and push it.
    vvvA to move the arm to the A button and push it.

In total, there are three shortest possible sequences of button presses on this directional keypad that would cause the robot to type 029A: <A^A>^^AvvvA, <A^A^>^AvvvA, and <A^A^^>AvvvA.

Unfortunately, the area containing this directional keypad remote control is currently experiencing high levels of radiation and nobody can go near it. A robot needs to be sent instead.

When the robot arrives at the directional keypad, its robot arm is pointed at the A button in the upper right corner. After that, a second, different directional keypad remote control is used to control this robot (in the same way as the first robot, except that this one is typing on a directional keypad instead of a numeric keypad).

There are multiple shortest possible sequences of directional keypad button presses that would cause this robot to tell the first robot to type 029A on the door. One such sequence is v<<A>>^A<A>AvA<^AA>A<vAAA>^A.

Unfortunately, the area containing this second directional keypad remote control is currently -40 degrees! Another robot will need to be sent to type on that directional keypad, too.

There are many shortest possible sequences of directional keypad button presses that would cause this robot to tell the second robot to tell the first robot to eventually type 029A on the door. One such sequence is <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A.

Unfortunately, the area containing this third directional keypad remote control is currently full of Historians, so no robots can find a clear path there. Instead, you will have to type this sequence yourself.

Were you to choose this sequence of button presses, here are all of the buttons that would be pressed on your directional keypad, the two robots' directional keypads, and the numeric keypad:

<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
v<<A>>^A<A>AvA<^AA>A<vAAA>^A
<A^A>^^AvvvA
029A

In summary, there are the following keypads:

    One directional keypad that you are using.
    Two directional keypads that robots are using.
    One numeric keypad (on a door) that a robot is using.
min
For each of these, here is a shortest sequence of button presses you could type to cause the desired code to be typed on the numeric keypad:

029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

The Historians are getting nervous; the ship computer doesn't remember whether the missing Historian is trapped in the area containing a giant electromagnet or molten lava. You'll need to make sure that for each of the five codes, you find the shortest sequence of button presses necessary.

The complexity of a single code (like 029A) is equal to the result of multiplying these two values:

    The length of the shortest sequence of button presses you need to type on your directional keypad in order to cause the code to be typed on the numeric keypad; for 029A, this would be 68.
    The numeric part of the code (ignoring leading zeroes); for 029A, this would be 29.

In the above example, complexity of the five codes can be found by calculating 68 * 29, 60 * 980, 68 * 179, 64 * 456, and 64 * 379. Adding these together produces 126384.

Find the fewest number of button presses you'll need to perform in order to cause the robot in front of the door to type each code. What is the sum of the complexities of the five codes on your list?

--- Part Two ---

Just as the missing Historian is released, The Historians realize that a second member of their search party has also been missing this entire time!

A quick life-form scan reveals the Historian is also trapped in a locked area of the ship. Due to a variety of hazards, robots are once again dispatched, forming another chain of remote control keypads managing robotic-arm-wielding robots.

This time, many more robots are involved. In summary, there are the following keypads:

    One directional keypad that you are using.
    25 directional keypads that robots are using.
    One numeric keypad (on a door) that a robot is using.

The keypads form a chain, just like before: your directional keypad controls a robot which is typing on a directional keypad which controls a robot which is typing on a directional keypad... and so on, ending with the robot which is typing on the numeric keypad.

The door codes are the same this time around; only the number of robots and directional keypads has changed.

Find the fewest number of button presses you'll need to perform in order to cause the robot in front of the door to type each code. What is the sum of the complexities of the five codes on your list?
*/

use std::{env, usize};
use std::fs::read_to_string;
use std::collections::HashMap;

fn lookup(keypad: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    let max_x = keypad.len();
    let max_y = keypad[0].len();
    for x in 0..max_x {
        for y in 0..max_y {
            if keypad[x][y] == c {
                return (x, y);
            }
        }
    }

    return (0, 0);
}

fn calc_seq(c1: char, c2: char, keypad: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let max_x = keypad.len();
    let max_y = keypad[0].len();
    let (source_x, source_y) = lookup(keypad, c1);
    let (target_x, target_y) = lookup(keypad, c2);

    // block distance, and anyhow the keypads are small
    // so KISS it
    let mut curr: Vec<(usize, usize, Vec<char>)> = vec![(source_x, source_y, vec![])];
    let mut seqs: Vec<Vec<char>> = Vec::new();
    while seqs.is_empty() {
        let mut next: Vec<(usize, usize, Vec<char>)> = Vec::new();
        for (x, y, v) in &curr {
            if *x == target_x && *y == target_y {
                let mut new_v = v.clone();
                new_v.push('A');
                seqs.push(new_v);
                continue;
            }

            for (dx, dy, dc) in [(0, 1, '>'), (1, 0, 'v'), (0, -1, '<'), (-1, 0, '^')] {
                let new_x = (*x as i32 + dx) as usize;
                let new_y: usize = (*y as i32 + dy) as usize;
                if new_x >= max_x || new_y >= max_y || keypad[new_x][new_y] == '#' {
                    continue;
                }

                let mut new_v = v.clone();
                new_v.push(dc);
                next.push((new_x, new_y, new_v));
            }
        }

        curr = next;
    }

    return seqs;
}

fn shortest_len(level: usize, c1: char, c2: char,
    weights: &HashMap<(char, char), Vec<Vec<char>>>,
    cache: &mut HashMap<(usize, char, char), usize>) -> usize {
    let key = (level, c1, c2);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    if level == 1 && weights.contains_key(&(c1, c2)) {
        let len = weights.get(&(c1, c2)).unwrap()[0].len();
        cache.insert(key, len);
        return *cache.get(&key).unwrap();
    }

    let mut len = usize::MAX;
    for s in weights.get(&(c1, c2)).unwrap() {
        let mut dis_len: usize = 0;
        for idx in 0..s.len() {
            let sc1 = if idx == 0 { 'A' } else { s[idx-1] };
            let sc2 = s[idx];

            dis_len += shortest_len(level - 1, sc1, sc2, weights, cache);
        }

        if len > dis_len {
            len = dis_len;
        }
    }

    cache.insert(key, len);
    return *cache.get(&key).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let level = if args.len() <= 2 { 25 } else { args[2].parse::<usize>().unwrap() };
    let input = read_to_string(filename).unwrap();

    // almost annoyingly simple input, this time
    let codes: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let num_keypad: Vec<Vec<char>> = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['#', '0', 'A'],
    ];
    let dir_keypad: Vec<Vec<char>> = vec![
        vec!['#', '^', 'A'],
        vec!['<', 'v', '>'],
    ];

    // then, idea: compute a map of how "expensive" (e.g., length) to go from one character to the next
    let mut num_weights: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    let mut dir_weights: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();

    let num_chars: Vec<char> = vec!['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let dir_chars: Vec<char> = vec!['A', '<', '>', 'v', '^'];
    // first iteration, with `num_keypad`
    for c1 in &num_chars {
        for c2 in &num_chars {
            let v = calc_seq(*c1, *c2, &num_keypad);

            num_weights.insert((*c1, *c2), v.clone());
        }
    }
    // first iteration, with `dir_keypad``
    for c1 in &dir_chars {
        for c2 in &dir_chars {
            let v = calc_seq(*c1, *c2, &dir_keypad);

            dir_weights.insert((*c1, *c2), v.clone());
        }
    }

    // filter weights, leave only a representative sequence
    let mut cache: HashMap<(usize, char, char), usize> = HashMap::new();

    // now easy, just sum up weight
    let mut sum = 0;
    for code in &codes {
        let mut weight: usize = 0;
        
        for i in 0..code.len() {
            let c1 = if i == 0 { 'A' } else { code[i-1] };
            let c2 = code[i];

            let mut min_len = usize::MAX;
            for s in num_weights.get(&(c1, c2)).unwrap() {
                let mut len = 0;
                for j in 0..s.len() {
                    let sc1 = if j == 0 { 'A' } else { s[j-1] };
                    let sc2 = s[j];
                    len += shortest_len(level, sc1, sc2, &dir_weights, &mut cache);
                }

                if min_len > len {
                    min_len = len;
                }
            }

            weight += min_len;
        }

        println!("weight: {}", weight);

        let num = weight * code.iter().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<usize>().unwrap();
        println!("num: {}", num);

        sum += num;
    }

    println!("sum: {}", sum);
}
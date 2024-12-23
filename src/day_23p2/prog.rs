/*
--- Day 23: LAN Party ---

As The Historians wander around a secure area at Easter Bunny HQ, you come across posters for a LAN party scheduled for today! Maybe you can find it; you connect to a nearby datalink port and download a map of the local network (your puzzle input).

The network map provides a list of every connection between two computers. For example:

kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn

Each line of text in the network map represents a single connection; the line kh-tc represents a connection between the computer named kh and the computer named tc. Connections aren't directional; tc-kh would mean exactly the same thing.

LAN parties typically involve multiplayer games, so maybe you can locate it by finding groups of connected computers. Start by looking for sets of three computers where each computer in the set is connected to the other two computers.

In this example, there are 12 such sets of three inter-connected computers:

aq,cg,yn
aq,vc,wq
co,de,ka
co,de,ta
co,ka,ta
de,ka,ta
kh,qp,ub
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn
ub,vc,wq

If the Chief Historian is here, and he's at the LAN party, it would be best to know that right away. You're pretty sure his computer's name starts with t, so consider only sets of three computers where at least one computer's name starts with t. That narrows the list down to 7 sets of three inter-connected computers:

co,de,ta
co,ka,ta
de,ka,ta
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn

Find all the sets of three inter-connected computers. How many contain at least one computer with a name that starts with t?

--- Part Two ---

There are still way too many results to go through them all. You'll have to find the LAN party another way and go there yourself.

Since it doesn't seem like any employees are around, you figure they must all be at the LAN party. If that's true, the LAN party will be the largest set of computers that are all connected to each other. That is, for each computer at the LAN party, that computer will have a connection to every other computer at the LAN party.

In the above example, the largest set of computers that are all connected to each other is made up of co, de, ka, and ta. Each computer in this set has a connection to every other computer in the set:

ka-co
ta-co
de-co
ta-ka
de-ta
ka-de

The LAN party posters say that the password to get into the LAN party is the name of every computer at the LAN party, sorted alphabetically, then joined together with commas. (The people running the LAN party are clearly a bunch of nerds.) In this example, the password would be co,de,ka,ta.

What is the password to get into the LAN party?
*/

use std::env;
use std::fs::read_to_string;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = read_to_string(filename).unwrap();

    let mut index = 1;
    let mut lookup: BTreeMap<String, u32> = BTreeMap::new();
    let mut graph: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    for line in input.lines() {
        let v: Vec<&str> = line.split("-").collect();
        let n1 = *lookup.entry(v[0].to_string()).or_insert({ index += 1; index });
        let n2 = *lookup.entry(v[1].to_string()).or_insert({ index += 1; index });
        graph.entry(n1).or_insert(BTreeSet::new()).insert(n2);
        graph.entry(n2).or_insert(BTreeSet::new()).insert(n1);
    }

    // calc sets of 2
    let mut sets: Vec<BTreeSet<u32>> = Vec::new();
    for (n1, neigh_n1) in &graph {
        for n2 in neigh_n1 {
            let s = BTreeSet::from([*n1, *n2]);
            sets.push(s);
        }
    }

    // calculate increasingly higher order sets
    // WARNING! slows as heck for input, because it is.. relatively well connected
    loop {
        // some optimizations
        let size: usize = sets.first().unwrap().len();
        // 1) eliminate dupes
        sets = Vec::from_iter(BTreeSet::from_iter(sets.iter().cloned()).iter().cloned());

        println!("{}, {}", size, sets.len());

        let mut next_sets: Vec<BTreeSet<u32>> = Vec::new();
        for i in 0..sets.len() {
            for j in i+1..sets.len() {
                let s1 = &sets[i];
                let s2 = &sets[j];

                let mut itr = s1.difference(&s2);
                let n1 = itr.next().unwrap();
                if !itr.next().is_none() {
                    continue;
                }

                let n2 = s2.difference(&s1).next().unwrap();
                if graph.get(n1).unwrap().contains(n2) {
                    let s: BTreeSet<u32> = BTreeSet::from_iter(s1.union(&s2).cloned());
                    next_sets.push(s);
                }
            }
        }

        if next_sets.is_empty() {
            break;
        } else {
            sets = next_sets;
        }
    }

    let reverse: BTreeMap<u32, String> = BTreeMap::from_iter(lookup.iter().map(|(k, v)| (*v, k.clone())));
    for s in sets {
        let mut v: Vec<String> = Vec::from_iter(s.iter().map(|n| reverse.get(n).unwrap().clone()));
        v.sort();
        println!("{}", v.join(","));
    }
}
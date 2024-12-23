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
*/

use std::env;
use std::fs::read_to_string;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = read_to_string(filename).unwrap();

    let mut graph: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for line in input.lines() {
        let v: Vec<&str> = line.split("-").collect();
        let n1 = v[0];
        let n2 = v[1];
        graph.entry(n1.to_string()).or_insert(BTreeSet::new()).insert(n2.to_string());
        graph.entry(n2.to_string()).or_insert(BTreeSet::new()).insert(n1.to_string());
    }

    // find sets of.. 3? exactly 3? assuming the former
    let mut sets: BTreeSet<String> = BTreeSet::new();
    for (n1, v) in &graph {
        if !n1.starts_with("t") {
            continue;
        }

        for n2 in v {
            for n3 in v {
                if graph.get(n2).unwrap().contains(n3) {
                    let mut v = vec![n1.to_string(), n2.to_string(), n3.to_string()];
                    v.sort();
                    sets.insert(v.join(""));
                }
            }
        }
    }

    println!("{}", sets.len());
}
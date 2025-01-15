use rusty_rubik::cube::CubeState;
use rusty_rubik::pruning::*;
use rusty_rubik::solver::IDASolver;
use rusty_rubik::CycleType;
use std::time::Instant;

fn main() {
    let cycle_type = CycleType {
        edge_partition: vec![(1, true), (4, true), (5, false)],
        corner_partition: vec![(1, true), (2, false), (3, true)],
    };
    // let cycle_type = CycleType {
    //     edge_partition: vec![(1, true), (5, true)],
    //     corner_partition: vec![(1, true), (3, true)],
    // };
    // CycleType {
    //     edge_partition: vec![(1, true), (5, true)],
    //     corner_partition: vec![(1, true), (1, true), (3, true)],
    // }
    // CycleType {
    //     edge_partition: vec![(1, true), (7, true)],
    //     corner_partition: vec![(1, true), (5, true)],
    // }
    // CycleType {
    //     edge_partition: vec![(1, true), (7, true)],
    //     corner_partition: vec![(1, true), (3, true)],
    // }

    let mut tag = "corners".to_string();
    for &(corner, orient) in cycle_type.corner_partition.iter() {
        tag.push_str(&format!("{}{}", corner, if orient { "o" } else { "n" }));
    }
    let pruning_tables = PruningTables::from(&tag);
    let now = Instant::now();
    let mut solver = IDASolver::new(CubeState::default(), &pruning_tables, cycle_type);
    let solution = solver.solve();
    let elapsed = now.elapsed();
    println!("{}", solution);
    println!("Elapsed: {:.2?}", elapsed);
}

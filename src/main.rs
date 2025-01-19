use rusty_rubik::cube::CubeState;
use rusty_rubik::pruning::*;
use rusty_rubik::solver::IDASolver;
use rusty_rubik::CycleType;
use std::time::Instant;

fn main() {
    let cycle_type = CycleType {
        corner_partition: vec![(1, true), (2, false), (3, true)],
        // edge_partition: vec![(2, true), (2, true)],
        ..Default::default()
    };

    let mut tag = "corners".to_string();
    for &(corner, orient) in cycle_type.corner_partition.iter() {
        tag.push_str(&format!("{}{}", corner, if orient { "o" } else { "n" }));
    }
    let pruning_tables = PruningTables::from(&tag, &cycle_type);
    let now = Instant::now();
    let mut solver = IDASolver::new(CubeState::default(), &pruning_tables, cycle_type);
    let solution = solver.solve();
    let elapsed = now.elapsed();
    println!("{}", solution);
    println!("Found phase 2 solution in {:.2?}", elapsed);
}

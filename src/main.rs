use rusty_rubik::cube::CubeState;
use rusty_rubik::pruning::*;
use rusty_rubik::solver::IDASolver;
use rusty_rubik::CycleType;

fn main() {
    let cycle_type = CycleType::from(vec![(1, true), (2, false), (3, true)]);
    let tag = "halfcorners".to_string();
    generate_pruning_table_corners(&tag, &cycle_type);
    // loop {
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     let input = input.trim();
    //     if input == "exit" {
    //         return;
    //     }
    //     let Ok(scramble) = parser::parse_scramble(input) else {
    //         continue;
    //     };
    //     let scramble = cube::MoveSequence::from(scramble);
    //     let solved = cube::CubeState::default();
    //     let twisted = solved.apply_move_instances(&scramble);
    //     println!("Twisted state: {:?}", twisted);
    // }

    // // load the pruning tables
    let pruning_tables = PruningTables::from(&tag);
    let mut solver = IDASolver::new(CubeState::default(), &pruning_tables, cycle_type);
    let solution = solver.solve();
    println!("{}", solution);
}

use rusty_rubik::cube;
use rusty_rubik::parser;
use rusty_rubik::pruning::generate_pruning_table_corners;
use rusty_rubik::CycleType;

fn main() {
    let cycle_type = CycleType::from(vec![(1, true), (2, false), (3, true)]);
    generate_pruning_table_corners(String::from("corners.pt"), &cycle_type);
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
    // let pruning_tables = PruningTables::default_tables();
    // let solver = IDASolver::new(new_state, &pruning_tables);
    // let solution = solver.solve();
    // println!("{}", solution);
    // println!("Verifying the above solution...");
    // let maybe_solved = new_state2.apply_move_instances(&solution);
    // if maybe_solved == cube::CubeState::default() {
    //     println!("Successfully verified!");
    // } else {
    //     println!("Uh oh...it's wrong...rip you.");
    // }
}

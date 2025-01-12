//! Contains utility methods of various puzzle solving methods.
//!
//! Includes A* search and iterative deepening A* (IDA*).

use crate::pruning::PruningTables;
use crate::{cube::*, CycleType};

/**
 * A generic solver trait.
 */
pub trait Solver {
    /// Gets a reference to the starting configuration.
    fn get_start_state(&self) -> &CubeState;

    /// Applies the solver-specific search algorithm to find a sequence
    /// of moves that transform the starting state into the solved state.
    fn solve(&self) -> MoveSequence;
}

/**
 * A solver implementing the iterative deepening A* search algorithm [Korf, 1997].
 *
 * This solver uses the pruning tables pre-computed in `pruning.rs`
 * to prevent the solver from exploring move sequences that will yield suboptimal
 * solutions. This is the method typically implemented in most optimal Rubik's Cube solvers.
 */
pub struct IDASolver<'a> {
    start_state: CubeState,
    pruning_tables: &'a PruningTables,
    target_cycle_type: CycleType<u8>,
}

enum SearchResult {
    Found,
    NewBound(u8),
}

impl<'a> IDASolver<'a> {
    pub fn new(
        start_state: CubeState,
        pruning_tables: &'a PruningTables,
        target_cycle_type: CycleType<u8>,
    ) -> Self {
        Self {
            start_state,
            pruning_tables,
            target_cycle_type,
        }
    }

    fn search_for_solution(
        &self,
        curr_path: &mut MoveSequence,
        last_state: &CubeState,
        g: u8,
        bound: u8,
    ) -> SearchResult {
        let last_h = self.pruning_tables.compute_h_value(last_state);
        let f = g + last_h;
        if f > bound {
            SearchResult::NewBound(f)
        } else if last_state.induces_corner_cycle_type(&self.target_cycle_type) {
            // yay it's solved!
            SearchResult::Found
        } else {
            let mut min = u8::MAX;
            let allowed_moves = curr_path.allowed_moves_after_seq();
            for m in ALL_MOVES
                .iter()
                .filter(|mo| ((1 << get_basemove_pos(mo.basemove)) & allowed_moves) == 0)
            {
                if !curr_path.is_empty() {
                    let last_move = curr_path[curr_path.len() - 1];
                    if last_move.basemove == m.basemove {
                        continue;
                    }
                }
                curr_path.push(*m);
                let next_state = last_state.apply_move_instance(m);
                let t = self.search_for_solution(curr_path, &next_state, g + 1, bound);
                match t {
                    SearchResult::Found => return SearchResult::Found,
                    SearchResult::NewBound(b) => {
                        min = std::cmp::min(b, min);
                    }
                };
                curr_path.pop();
            }
            SearchResult::NewBound(min)
        }
    }
}

impl Solver for IDASolver<'_> {
    fn get_start_state(&self) -> &CubeState {
        &self.start_state
    }

    fn solve(&self) -> MoveSequence {
        let start_state = self.get_start_state();

        // initial lower bound on number of moves needed to solve start state
        let mut bound = self.pruning_tables.compute_h_value(start_state);
        let mut path: MoveSequence = MoveSequence::default();
        loop {
            println!("Searching depth {}...", bound);
            match self.search_for_solution(&mut path, start_state, 0, bound) {
                SearchResult::Found => {
                    break;
                }
                SearchResult::NewBound(t) => {
                    bound = t;
                }
            }
        }
        path
    }
}

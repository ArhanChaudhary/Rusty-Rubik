//! Contains utility methods of various puzzle solving methods.
//!
//! Includes A* search and iterative deepening A* (IDA*).

use crate::cube::*;
use crate::pruning::PruningTables;
use std::collections::HashMap;
use std::collections::HashSet;

use priority_queue::PriorityQueue;

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
 * A solver implementing the A* search algorithm.
 *
 * This solver is only able to handle short, small-depth scrambles due
 * to the massive space usage of A* search and similar BFS-style search algorithms.
 * Thus, we strongly recommend using IDASolver instead.
 */
pub struct AStarSolver {
    start_state: CubeState,
}

impl AStarSolver {
    pub fn new(state: CubeState) -> Self {
        AStarSolver { start_state: state }
    }
}

impl Solver for AStarSolver {
    fn get_start_state(&self) -> &CubeState {
        &self.start_state
    }

    fn solve(&self) -> MoveSequence {
        let mut queue = PriorityQueue::new();
        let mut visited = HashSet::<CubeState>::new();
        let mut come_from = HashMap::<CubeState, (CubeState, MoveInstance)>::new();
        let mut g_scores = HashMap::<CubeState, i32>::new();

        // TODO: need to compress cube state
        queue.push(self.get_start_state().clone(), 0);
        g_scores.insert(self.get_start_state().clone(), 0);
        while !queue.is_empty() {
            if let Some((current, priority)) = queue.pop() {
                if current == CubeState::default() {
                    // we found the solved state!
                    break;
                }
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current.clone());
                // iterate through all moves
                for m in ALL_MOVES.iter() {
                    let new_state = current.apply_move_instance(m);
                    let new_g_score = priority - 1;
                    let neighbor_g_score = g_scores.get(&new_state).unwrap_or(&i32::MIN);
                    if new_g_score > *neighbor_g_score {
                        come_from.insert(new_state.clone(), (current.clone(), *m));
                        g_scores.insert(new_state.clone(), new_g_score);
                    }
                    if queue.get(&new_state).is_none() {
                        queue.push(new_state, priority - 1);
                    } else if let Some((_, p)) = queue.get(&new_state) {
                        if *p < priority - 1 {
                            queue.push(new_state, priority - 1);
                        }
                    }
                }
            }
        }
        // now reconstruct the path
        let mut curr = CubeState::default();
        let mut path = vec![];
        while curr != self.get_start_state().clone() {
            if let Some((c, m)) = come_from.get(&curr) {
                path.push(*m);
                curr = c.clone();
            }
        }
        path.reverse();
        MoveSequence(path)
    }
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
}

enum SearchResult {
    Found,
    NewBound(u8),
}

impl<'a> IDASolver<'a> {
    pub fn new(state: CubeState, tables: &'a PruningTables) -> Self {
        Self {
            start_state: state,
            pruning_tables: tables,
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
        } else if *last_state == CubeState::default() {
            // yay it's solved!
            SearchResult::Found
        } else {
            let mut min = u8::MAX;
            let allowed_moves = curr_path.allowed_moves_after_seq();
            for m in ALL_MOVES
                .iter()
                .filter(|mo| ((1 << get_basemove_pos(mo.basemove)) & allowed_moves) == 0)
            {
                if !curr_path.get_moves().is_empty() {
                    let path = curr_path.get_moves_mut();
                    let last_move = path[path.len() - 1];
                    if last_move.basemove == m.basemove {
                        continue;
                    }
                }
                curr_path.get_moves_mut().push(*m);
                let next_state = last_state.apply_move_instance(m);
                let t = self.search_for_solution(curr_path, &next_state, g + 1, bound);
                match t {
                    SearchResult::Found => return SearchResult::Found,
                    SearchResult::NewBound(b) => {
                        min = std::cmp::min(b, min);
                    }
                };
                curr_path.get_moves_mut().pop();
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
        let mut path: MoveSequence = MoveSequence(vec![]);
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

//! A module for representing and constructing pruning tables.
//!
//! At a high level, a pruning tables gives a lower bound on how many
//! moves are needed to solve a given position of the Rubik's Cube. The intention
//! is for these tables to be pre-generated before any solving work begins.
//!
//! Each table is generated by executing an iterative deepening DFS (IDDFS) starting
//! from the solved state. For each state, the depth is recorded in a vector
//! of the appropriate size.

use crate::cube::*;
use std::io::Write;

/**
 * A struct holding pruning information for certain subgroups of the
 * Rubik's Cube.
 *
 * Each pruning table provides a lower bound on how many moves are
 * needed to transform a given state into the solved state within each subgroup.
 * These tables are obtained from `pruning.rs`.
 */
pub struct PruningTables {
    /// A pruning table representing the subgroup of corner permutation and orientation.
    corners: Vec<u8>,
    // A pruning table representing the subgroup of edge orientation.
    // pub eo: Vec<u8>,
    // A pruning table representing the subgroup of edge permutation.
    // pub ep: Vec<u8>,
}

impl PruningTables {
    /// Reads the default pruning tables from the default
    /// file names.
    pub fn default_tables() -> Self {
        let corners = std::fs::read("corners.pt").unwrap();
        // let edges_o = std::fs::read("edges_o.pt").unwrap();
        // let edges_p = std::fs::read("edges_p.pt").unwrap();
        PruningTables {
            corners,
            // eo: edges_o,
            // ep: edges_p,
        }
    }

    pub fn corners(&self, index: usize) -> u8 {
        self.corners[index]
    }

    /// Computes a lower bound on the number of moves needed to
    /// solve the given state, based on the pruning table values.
    pub fn compute_h_value(&self, state: &CubeState) -> u8 {
        let corners = state.corner_state_index();
        // let (corners, eo, ep) = state.state_index();
        // std::cmp::max(
        //     self.corners[corners as usize],
        //     std::cmp::max(self.eo[eo as usize], self.ep[ep as usize]),
        // )
        self.corners[corners as usize]
    }
}

/// A wrapper function around the main logic of IDDFS.
fn iddfs(
    goal_states: &[CubeState],
    depth: u8,
    bv: &mut [u8],
    prop_func: &dyn Fn(&CubeState) -> usize,
    tag: String,
) {
    if depth < 1 {
        panic!("Depth must be positive");
    }
    for d in 1..depth {
        println!("Building {} pruning table for depth {}...", tag, d);
        for goal_state in goal_states {
            iddfs_search(goal_state, d, d, bv, 0, &prop_func);
        }
        println!(
            "{} entries remaining at depth {}",
            bv.iter().filter(|&&x| x == 0).count(),
            d
        );
    }
}

/// Starts a depth-bounded DFS from the given state.
fn iddfs_search(
    state: &CubeState,
    original_depth: u8,
    d: u8,
    bv: &mut [u8],
    allowed_moves: u8,
    prop_func: &dyn Fn(&CubeState) -> usize,
) {
    if d == 0 {
        // cool, we've hit the desired depth now
        let index = prop_func(state);
        if index > 0 && bv[index] == 0 {
            bv[index] = original_depth;
        }
    } else {
        for m in ALL_MOVES
            .iter()
            .filter(|&mo| (1 << get_basemove_pos(mo.basemove)) & allowed_moves == 0)
        {
            let new_state = state.apply_move_instance(m);
            let index = prop_func(&new_state);
            if index > 0 && bv[index] != 0 && bv[index] < original_depth - d + 1 {
                continue;
            }
            let new_allowed_moves = get_allowed_post_moves(allowed_moves, Some(m.basemove));
            iddfs_search(
                &new_state,
                original_depth,
                d - 1,
                bv,
                new_allowed_moves,
                &prop_func,
            );
        }
    }
}

fn write_table(table: &[u8], filename: String) {
    let mut file = std::fs::File::create(filename).expect("Unable to create file.");
    file.write_all(table).expect("Unable to write to file.");
}

/// Generates a pruning table for the corners of a Rubik's Cube.
pub fn generate_pruning_table_corners(filename: String) {
    let goal_states = vec![CubeState::default()];
    let mut table = vec![0_u8; 88179840];
    iddfs(
        &goal_states,
        9,
        &mut table,
        &|state: &CubeState| {
            // let (corner, _, _) = state.state_index();
            // corner as usize
            state.corner_state_index() as usize
        },
        String::from("corners"),
    );
    write_table(&table, filename);
}

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Add, Rem};
use std::sync::LazyLock;

/**
pub struct CubeState {
}
is an example for a 3x3

have a macro to generate these states?
*/

pub trait PuzzleMove {}

/// General trait to describe a twisty puzzle.
pub trait Puzzle {
    type M;

    // Initializes this puzzle in its solved state.
    fn new() -> Self;

    fn get_all_moves() -> HashSet<Self::M>;

    // TODO: Megaminx has different scrambling and actual solving moves

    // Generally, for puzzles like NxN puzzles, all available moves are allowed.
    // However, for bandaging puzzles such as the Bandaged Cube and Square-1, certain
    // moves will restrict others from being allowed.
    fn get_allowed_moves(&self) -> HashSet<Self::M>;

    fn make_move(&self, m: Self::M) -> Self;
    fn make_move_mut(&mut self, m: Self::M) -> &Self;
}

#[derive(Clone)]
pub struct Cube3 {
    pub cp: [u8; 8],
    pub co: [u8; 8],
    pub ep: [u8; 12],
    pub eo: [u8; 12],
}

pub struct Cube4 {
    pub cp: [u8; 8],
    pub co: [i8; 8],
    pub wings1: [u8; 24],
    pub xcenters1: [u8; 24],
}

pub struct Cube5 {
    // corners are always the same...
    pub wings1: [u8; 24],
    pub midges_ep: [u8; 12],
    pub midges_eo: [i8; 12],
    pub xcenters1: [u8; 24],
    pub pluscenters1: [u8; 24],
}

pub struct Cube6 {
    //...
    pub wings1: [u8; 24],
    pub wings2: [u8; 24],
    // ...
}

// pub struct Cube335 {
//     pub cp: [u8, 8],
//     // actually in cuboids, where 90 degree rotation isn't allowed, orientation doesn't
//     // matter
//     pub
// }

// macro_rules! define_cube {
//     ($dim:literal) => {
//         pub struct Cube$dim {
//             pub cp: [u8; 8],
//             pub co: [i8; 8],

//         }
//     }
// }

#[derive(Debug, EnumIter, Eq, PartialEq, Hash, Copy, Clone, FromPrimitive)]
pub enum CubeAxis {
    U,
    D,
    L,
    R,
    F,
    B,
}

// rotations?

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CubeMove {
    pub axis: u8,
    pub depth: u8,
    pub rotation: u8,
}

impl PuzzleMove for CubeMove {}

impl CubeMove {
    fn new(axis: u8, depth: u8, rotation: u8) -> Self {
        CubeMove {
            axis,
            depth,
            rotation,
        }
    }
}

impl Default for Cube3 {
    fn default() -> Self {
        Self {
            cp: [0, 1, 2, 3, 4, 5, 6, 7],
            co: [0_u8; 8],
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            eo: [0_u8; 12],
        }
    }
}

// move actions?

pub fn apply_permutation<T: Clone + Copy, const N: usize>(
    og_state: [T; N],
    delta: &[usize; N],
    count: u8,
) -> [T; N] {
    let mut new_array = og_state;
    for _ in 0..count {
        for i in 0..N {
            new_array[delta[i]] = og_state[i];
        }
    }
    new_array
}

pub fn apply_orientation<T: Clone + Copy + Add<Output = T> + Rem<Output = T>, const N: usize>(
    og_state: [T; N],
    delta: &[T; N],
    num_orientations: T,
    count: u8,
) -> [T; N] {
    let mut new_array = og_state;
    for _ in 0..count {
        for i in 0..N {
            new_array[i] = (og_state[i] + delta[i]) % num_orientations;
        }
    }
    new_array
}

impl Puzzle for Cube3 {
    type M = CubeMove;

    fn new() -> Self {
        Cube3::default()
    }

    fn get_all_moves() -> HashSet<Self::M> {
        let mut all_moves = HashSet::new();
        for axis in CubeAxis::iter() {
            for rotation in 0..3 {
                all_moves.insert(CubeMove {
                    axis: axis as u8,
                    depth: 1,
                    rotation,
                });
            }
        }
        all_moves
    }

    fn get_allowed_moves(&self) -> HashSet<Self::M> {
        <Self as Puzzle>::get_all_moves()
    }

    fn make_move(&self, m: CubeMove) -> Self {
        let axis = <CubeAxis as FromPrimitive>::from_u8(m.axis).unwrap();
        Cube3 {
            cp: apply_permutation(self.cp, CP_DELTAS.get(&axis).unwrap(), m.rotation),
            co: apply_orientation(self.co, CO_DELTAS.get(&axis).unwrap(), 3, m.rotation),
            ep: apply_permutation(self.ep, EP_DELTAS.get(&axis).unwrap(), m.rotation),
            eo: apply_orientation(self.eo, EO_DELTAS.get(&axis).unwrap(), 2, m.rotation),
        }
    }

    fn make_move_mut(&mut self, m: CubeMove) -> &Self {
        *self = self.make_move(m);
        self
    }
}

macro_rules! hashmap {
    ($cnt:ty,$($key:expr => $value:expr),+ ,) => {
        {
            let mut m = HashMap::<CubeAxis, $cnt>::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };
}

static CP_DELTAS: LazyLock<HashMap<CubeAxis, [usize; 8]>> = LazyLock::new(|| {
    hashmap! {
        [usize; 8],
        CubeAxis::U => [1, 2, 3, 0, 4, 5, 6, 7],
        CubeAxis::D => [0, 1, 2, 3, 5, 6, 7, 4],
        CubeAxis::R => [0, 6, 1, 3, 4, 2, 5, 7],
        CubeAxis::L => [3, 1, 2, 4, 7, 5, 6, 0],
        CubeAxis::F => [0, 1, 5, 2, 3, 4, 6, 7],
        CubeAxis::B => [7, 0, 2, 3, 4, 5, 1, 6],
    }
});

static CO_DELTAS: LazyLock<HashMap<CubeAxis, [u8; 8]>> = LazyLock::new(|| {
    hashmap! {
        [u8; 8],
        CubeAxis::U => [0, 0, 0, 0, 0, 0, 0, 0],
        CubeAxis::D => [0, 0, 0, 0, 0, 0, 0, 0],
        CubeAxis::R => [0, 2, 1, 0, 0, 2, 1, 0],
        CubeAxis::L => [1, 0, 0, 2, 1, 0, 0, 2],
        CubeAxis::F =>[0, 0, 2, 1, 2, 1, 0, 0],
        CubeAxis::B => [2, 1, 0, 0, 0, 0, 2, 1],
    }
});

static EP_DELTAS: LazyLock<HashMap<CubeAxis, [usize; 12]>> = LazyLock::new(|| {
    hashmap! {
        [usize; 12],
        CubeAxis::U =>[1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11],
        CubeAxis::D =>[0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
        CubeAxis::R =>[0, 5, 2, 3, 4, 9, 1, 7, 8, 6, 10, 11],
        CubeAxis::L =>[0, 1, 2, 7, 3, 5, 6, 11, 8, 9, 10, 4],
        CubeAxis::F =>[0, 1, 6, 3, 4, 5, 8, 2, 7, 9, 10, 11],
        CubeAxis::B =>[4, 1, 2, 3, 10, 0, 6, 7, 8, 9, 5, 11],
    }
});

static EO_DELTAS: LazyLock<HashMap<CubeAxis, [u8; 12]>> = LazyLock::new(|| {
    hashmap! {
        [u8; 12],
        CubeAxis::U =>[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        CubeAxis::D =>[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        CubeAxis::R =>[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        CubeAxis::L =>[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        CubeAxis::F =>[0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        CubeAxis::B =>[1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0],
    }
});

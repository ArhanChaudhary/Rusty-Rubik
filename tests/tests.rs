#[cfg(test)]
mod tests {
    use rusty_rubik::cube::*;
    use rusty_rubik::parser::*;
    // PARSER TESTS
    #[test]
    fn parse_single_move() {
        assert_eq!(
            parse_scramble("U").unwrap()[0],
            MoveInstance {
                basemove: BaseMoveToken::U,
                dir: Direction::Normal,
            }
        );
    }

    #[test]
    fn parse_single_move_with_spaces() {
        assert_eq!(
            parse_scramble("U   \t").unwrap()[0],
            MoveInstance {
                basemove: BaseMoveToken::U,
                dir: Direction::Normal,
            }
        );
    }

    #[test]
    fn parse_multi_moves() {
        assert_eq!(
            parse_scramble("U2 F'").unwrap()[0],
            MoveInstance {
                basemove: BaseMoveToken::U,
                dir: Direction::Double,
            }
        );
        assert_eq!(
            parse_scramble("U2 F'").unwrap()[1],
            MoveInstance {
                basemove: BaseMoveToken::F,
                dir: Direction::Prime,
            }
        )
    }

    // CUBE STRUCTURE TESTS

    #[test]
    fn create_new_move_instance() {
        let move_instance = MoveInstance::new(BaseMoveToken::F, Direction::Prime);
        assert_eq!(move_instance.basemove, BaseMoveToken::F);
        assert_eq!(move_instance.dir, Direction::Prime)
    }

    #[test]
    fn index_of_solved_state() {
        // let (c, eo, ep) = CubeState::default().state_index();
        let c = CubeState::default().corner_state_index();
        assert_eq!(c, 0);
        // assert_eq!(eo, 0);
        // assert_eq!(ep, 0);
    }
}

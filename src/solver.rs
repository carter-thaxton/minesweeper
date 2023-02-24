
use crate::game::{GameMove, GridState, MinesweeperGame};

pub fn get_next_move(game: &MinesweeperGame) -> GameMove {
    // no moves possible
    if game.game_over() {
        return GameMove::NoOp;
    }

    let w = game.width();
    let h = game.height();

    // special case for first move: just pick the middle
    if game.revealed_count() == 0 {
        return GameMove::Reveal(w/2, h/2);
    }

    // first check for any logically consistent moves
    for y in 0..h {
        for x in 0..w {
            let state = game.peek_at(x, y, false);
            //println!("=== Point: {x},{y} - {state:?}");

            if let GridState::Count(count) = state {
                if let Some(m) = logical_move_around_count(x, y, count, game) {
                    return m;
                }
            }
        }
    }

    // TODO: now we need to guess...

    GameMove::NoOp
}


fn neighbors(x: u32, y: u32, w: u32, h: u32) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    for y2 in y..=y + 2 {
        if y2 > 0 && y2 <= h {
            for x2 in x..=x + 2 {
                if x2 > 0 && x2 <= w {
                    result.push((x2-1, y2-1));
                }
            }
        }
    }
    result
}


fn logical_move_around_count(x: u32, y: u32, count: u8, game: &MinesweeperGame) -> Option<GameMove> {
    let w = game.width();
    let h = game.height();

    // count the unrevealed positions around a point, as well as any existing flags
    let mut flag_count = 0;
    let mut unrevealed_count = 0;
    let mut unrevealed_pos: Option<(u32, u32)> = None;

    for (x2, y2) in neighbors(x, y, w, h) {
        let neighbor_state = game.peek_at(x2, y2, false);
        match neighbor_state {
            GridState::Empty | GridState::Count(_) => {},
            GridState::Flagged => { flag_count += 1; }
            GridState::Unrevealed => {
                unrevealed_count += 1;
                unrevealed_pos.get_or_insert((x2, y2));
            }
            _ => { return None; }
        }
    }

    if let Some((x2, y2)) = unrevealed_pos {
        // if the count on this point matches the number of unresolved neighbors plus any existing flags, then we can flag this point
        if count == unrevealed_count + flag_count {
            return Some(GameMove::Flag(x2, y2));
        }

        // if the count on this point already matches the number of flags around it, then we can reveal the remaining unresolved neighbors
        if count == flag_count {
            return Some(GameMove::Reveal(x2, y2));
        }
    }

    None
}

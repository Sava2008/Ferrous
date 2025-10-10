use crate::constants::{BOARD_AREA, BOARD_SIDE, SQUARE_SIDE};
use crate::game_logic::{
    board::Board,
    pieces::{ChessPiece, Piece, Void},
    state_enums::{KingChecked, PieceColor, PieceVariant},
};

use ggez::{
    Context, GameResult,
    graphics::{GlBackendSpec, Image, ImageGeneric},
    mint::Point2,
};
use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub const fn generate_coords() -> [(u8, u8); BOARD_AREA] {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut idx: usize = 0;
    let mut coords: [(u8, u8); BOARD_AREA] = [(69, 69); BOARD_AREA];
    while y < BOARD_SIDE {
        coords[idx] = (y, x);
        idx += 1;
        x += 1;
        if x == BOARD_SIDE {
            y += 1;
            x = 0;
        }
    }

    return coords;
}

pub fn generate_empty_board() -> [ChessPiece; BOARD_AREA] {
    return std::array::from_fn(|_| ChessPiece::Square(Void {}));
}

pub fn load_images(
    ctx: &mut Context,
) -> GameResult<HashMap<(PieceColor, PieceVariant), ImageGeneric<GlBackendSpec>>> {
    return Ok(HashMap::from([
        (
            (PieceColor::Black, PieceVariant::B),
            Image::new(ctx, "/black_bishop.png")?,
        ),
        (
            (PieceColor::Black, PieceVariant::K),
            Image::new(ctx, "/black_king.png")?,
        ),
        (
            (PieceColor::Black, PieceVariant::N),
            Image::new(ctx, "/black_knight.png")?,
        ),
        (
            (PieceColor::Black, PieceVariant::P),
            Image::new(ctx, "/black_pawn.png")?,
        ),
        (
            (PieceColor::Black, PieceVariant::Q),
            Image::new(ctx, "/black_queen.png")?,
        ),
        (
            (PieceColor::Black, PieceVariant::R),
            Image::new(ctx, "/black_rook.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::B),
            Image::new(ctx, "/white_bishop.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::K),
            Image::new(ctx, "/white_king.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::N),
            Image::new(ctx, "/white_knight.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::P),
            Image::new(ctx, "/white_pawn.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::Q),
            Image::new(ctx, "/white_queen.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::R),
            Image::new(ctx, "/white_rook.png")?,
        ),
    ]));
}

pub fn coords_to_index(coords: Point2<f32>) -> Option<usize> {
    return Some(
        coords.y as usize / SQUARE_SIDE as usize * 8 + coords.x as usize / SQUARE_SIDE as usize,
    );
}

pub fn index_to_coords(index: usize) -> (u8, u8) {
    return (index as u8 / 8, index as u8 % 8);
}

pub fn is_diagonal(idx1: usize, king_idx: usize) -> bool {
    let coords1: (u8, u8) = index_to_coords(idx1);
    let coords2: (u8, u8) = index_to_coords(king_idx);

    return max(coords1.0, coords2.0) - min(coords1.0, coords2.0)
        == max(coords1.1, coords2.1) - min(coords1.1, coords2.1);
}

pub fn is_line(idx1: usize, king_idx: usize) -> bool {
    let (x1, y1) = index_to_coords(idx1);
    let (x2, y2) = index_to_coords(king_idx);

    return x1 == x2 || y1 == y2;
}

pub fn is_adjancent_file(idx1: usize, idx2: usize) -> bool {
    let (row1, row2) = (idx1 % 8, idx2 % 8);
    return max(row1, row2) - min(row1, row2) <= 1;
}

pub fn i8_coords_to_index(coords: (i8, i8)) -> usize {
    return coords.0 as usize * 8 + coords.1 as usize;
}

pub fn calculate_path_between(attcker_idx: usize, king_idx: usize) -> Vec<usize> {
    let mut path: Vec<usize> = Vec::new();
    let (mut greatest, least) = (max(attcker_idx, king_idx), min(attcker_idx, king_idx));
    if is_diagonal(attcker_idx, king_idx) {
        if (greatest - least) % 7 == 0 {
            loop {
                if greatest - 7 <= least {
                    break;
                }
                greatest -= 7;
                path.push(greatest);
            }
        } else {
            loop {
                if greatest - 9 <= least {
                    break;
                }
                greatest -= 9;
                path.push(greatest);
            }
        }
    } else {
        if (greatest - least) % 8 == 0 {
            loop {
                if greatest - 8 <= least {
                    break;
                }
                greatest -= 8;
                path.push(greatest);
            }
        } else {
            loop {
                if greatest - 1 <= least {
                    break;
                }
                greatest -= 1;
                path.push(greatest);
            }
        }
    }
    path.push(attcker_idx);
    return path;
}

fn opposite_direction(pinned: usize, king_idx: usize) -> Result<Vec<usize>, String> {
    let mut opposite_path: Vec<usize> = Vec::new();
    let step: isize;

    if is_diagonal(pinned, king_idx) {
        step = if (pinned as isize - king_idx as isize) % 7 == 0 {
            if pinned > king_idx { 7 } else { -7 }
        } else {
            if pinned > king_idx { 9 } else { -9 }
        };
    } else if is_line(pinned, king_idx) {
        step = if (pinned as isize - king_idx as isize) % 8 == 0 {
            if pinned > king_idx { 8 } else { -8 }
        } else {
            if pinned > king_idx { 1 } else { -1 }
        };
    } else {
        return Err(format!("{pinned} and {king_idx} are not aligned"));
    }
    let mut current: isize = pinned as isize;
    loop {
        current += step;
        if current < 0 || current > 63 {
            break;
        }

        let file_difference: isize = ((pinned % 8) as isize - (current % 8) as isize).abs();
        let rank_difference: isize = ((pinned / 8) as isize - (current / 8) as isize).abs();
        if file_difference != rank_difference && file_difference != 0 && rank_difference != 0 {
            break;
        }

        opposite_path.push(current as usize);
    }

    return Ok(opposite_path);
}

// no idea how works
fn is_pinned<T: Piece>(piece: &T, king_idx: usize, board: &Board) -> bool {
    let alignment: i8 = if is_diagonal(king_idx, piece.index()) {
        1
    } else if is_line(king_idx, piece.index()) {
        2
    } else {
        return false;
    };
    let temp_path: Vec<usize> = if let Ok(range) = opposite_direction(piece.index(), king_idx) {
        range
    } else {
        return false;
    };
    let color: PieceColor = match piece.color() {
        PieceColor::Black => PieceColor::Black,
        PieceColor::White => PieceColor::White,
    };

    for i in temp_path.iter() {
        if match alignment {
            1 => board.squares[*i].is_diagonal_attacker(color),
            2 => board.squares[*i].is_linear_attacker(color),
            _ => unreachable!(),
        } {
            return true;
        } else if board.squares[*i].is_void() {
            continue;
        } else {
            return false;
        }
    }

    return false;
}

pub fn generate_legal_moves<T: Piece>(
    piece: &T,
    board: &Board,
    king_idx: usize,
    checked: &(KingChecked, Option<usize>, Option<usize>),
    en_peasant_target: Option<usize>,
) -> GameResult<Vec<usize>> {
    match checked {
        (_, None, None) => {
            let legal_moves: Vec<usize> = piece.legal_moves(&board, en_peasant_target);
            let temp_path: Vec<usize> = calculate_path_between(king_idx, piece.index());
            let alignment: i8 = if is_diagonal(king_idx, piece.index()) {
                1
            } else if is_line(king_idx, piece.index()) {
                2
            } else {
                return Ok(legal_moves);
            };
            if temp_path
                .iter()
                .take_while(|x: &&usize| *x != &king_idx)
                .any(|x: &usize| board.squares[*x].is_piece())
            {
                return Ok(legal_moves);
            } else {
                let mut opposite_path: Vec<usize> =
                    if let Ok(range) = opposite_direction(piece.index(), king_idx) {
                        range
                    } else {
                        return Ok(legal_moves);
                    };
                let color = match piece.color() {
                    PieceColor::Black => PieceColor::Black,
                    PieceColor::White => PieceColor::White,
                };
                for i in opposite_path.iter() {
                    if match alignment {
                        1 => board.squares[*i].is_diagonal_attacker(color),
                        2 => board.squares[*i].is_linear_attacker(color),
                        _ => unreachable!(),
                    } {
                        let mut full_path: Vec<usize> =
                            calculate_path_between(piece.index(), king_idx);
                        full_path.append(&mut opposite_path);
                        return Ok(legal_moves
                            .iter()
                            .filter(|i: &&usize| full_path.iter().any(|j: &usize| j == *i))
                            .map(|el: &usize| *el)
                            .collect());
                    } else {
                        if board.squares[*i].is_piece() {
                            return Ok(legal_moves);
                        }
                    }
                }
            }
            return Ok(legal_moves);
        }
        (_, Some(_), Some(_)) => return Ok(Vec::new()),
        (_, Some(i), None) => match board.squares[*i] {
            ChessPiece::P(_) | ChessPiece::N(_) => {
                if is_pinned(piece, king_idx, &board) {
                    return Ok(Vec::new());
                }
                return Ok(piece
                    .legal_moves(&board, en_peasant_target)
                    .into_iter()
                    .filter(|idx: &usize| idx == i)
                    .collect());
            }
            ChessPiece::B(_) => {
                if is_pinned(piece, king_idx, &board) {
                    return Ok(Vec::new());
                }
                let path: Vec<usize> = calculate_path_between(*i, king_idx);
                return Ok(piece
                    .legal_moves(&board, en_peasant_target)
                    .into_iter()
                    .filter(|idx: &usize| path.iter().any(|x: &usize| x == idx))
                    .collect());
            }
            ChessPiece::R(_) => {
                if is_pinned(piece, king_idx, &board) {
                    return Ok(Vec::new());
                }
                let path: Vec<usize> = calculate_path_between(*i, king_idx);
                return Ok(piece
                    .legal_moves(&board, en_peasant_target)
                    .into_iter()
                    .filter(|idx: &usize| path.iter().any(|x: &usize| x == idx))
                    .collect());
            }
            ChessPiece::Q(_) => {
                if is_pinned(piece, king_idx, &board) {
                    return Ok(Vec::new());
                }
                let path: Vec<usize> = calculate_path_between(*i, king_idx);
                return Ok(piece
                    .legal_moves(&board, en_peasant_target)
                    .into_iter()
                    .filter(|idx: &usize| path.iter().any(|x: &usize| x == idx))
                    .collect());
            }
            _ => return Ok(Vec::new()),
        },
        (_, None, Some(_)) => unreachable!(),
    };
}

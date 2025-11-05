use ggez::GameResult;
use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use crate::{
    constants::{BOARD_AREA, PAWN_VALUE, ROOK_VALUE},
    game_logic::{
        PieceColor,
        pieces::{Bishop, ChessPiece, King, Knight, Pawn, Queen, Rook, Void},
        state_enums::{GameMode, KingChecked, PieceVariant},
    },
    helper_functions::generate_empty_board,
};

#[derive(Debug, PartialEq, Eq)]
pub struct MoveCancellation {
    pub moved_piece: Option<(u8, usize)>,
    pub check_state: (KingChecked, Option<usize>, Option<usize>),
    pub captured_piece: Option<(u8, usize, PieceVariant)>,
    pub en_peasant: Option<usize>,
    pub castled_rook: Option<(u8, usize, usize)>, // (ID, initial_pos, final_pos)
    pub promoted_pawn: Option<(u8, usize)>,
    pub whose_turn: Option<PieceColor>,
    pub was_moved: bool,
    pub previous_gamemode: Option<GameMode>,
}
impl MoveCancellation {
    pub fn new() -> Self {
        return MoveCancellation {
            moved_piece: None,
            check_state: (KingChecked::None, None, None),
            captured_piece: None,
            en_peasant: None,
            castled_rook: None,
            promoted_pawn: None,
            whose_turn: None,
            was_moved: false,
            previous_gamemode: None,
        };
    }
}
impl Clone for MoveCancellation {
    fn clone(&self) -> Self {
        return MoveCancellation {
            moved_piece: self.moved_piece.clone(),
            check_state: self.check_state.clone(),
            captured_piece: self.captured_piece.clone(),
            en_peasant: self.en_peasant.clone(),
            castled_rook: self.castled_rook.clone(),
            promoted_pawn: self.promoted_pawn.clone(),
            whose_turn: self.whose_turn.clone(),
            was_moved: self.was_moved.clone(),
            previous_gamemode: self.previous_gamemode.clone(),
        };
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Board {
    pub squares: [ChessPiece; BOARD_AREA],
    pub white_locations: HashMap<u8, usize>,
    pub black_locations: HashMap<u8, usize>,
    pub white_vision: HashSet<usize>,
    pub black_vision: HashSet<usize>,
    pub checked: KingChecked,
    pub gamemode: GameMode,
    pub en_peasant_susceptible: Option<usize>,
    pub check: (KingChecked, Option<usize>, Option<usize>),
    pub chosen_piece: Option<usize>,
    pub dest_square: Option<usize>,
    pub engine_move: Option<(usize, usize)>,
    pub legal_moves: Vec<usize>,
    pub move_history: Vec<MoveCancellation>,
}
impl Clone for Board {
    fn clone(&self) -> Self {
        return Board {
            squares: self.squares.clone(),
            white_locations: self.white_locations.clone(),
            black_locations: self.black_locations.clone(),
            white_vision: self.white_vision.clone(),
            black_vision: self.black_vision.clone(),
            checked: self.checked.clone(),
            gamemode: self.gamemode.clone(),
            en_peasant_susceptible: self.en_peasant_susceptible.clone(),
            check: self.check.clone(),
            chosen_piece: self.chosen_piece.clone(),
            dest_square: self.dest_square.clone(),
            engine_move: self.engine_move.clone(),
            legal_moves: self.legal_moves.clone(),
            move_history: self.move_history.clone(),
        };
    }
}

impl Board {
    pub fn new() -> GameResult<Self> {
        return Ok(Board {
            squares: generate_empty_board(),
            white_locations: HashMap::new(),
            black_locations: HashMap::new(),
            white_vision: HashSet::new(),
            black_vision: HashSet::new(),
            checked: KingChecked::None,
            gamemode: GameMode::SelectionWhite,
            en_peasant_susceptible: None,
            check: (KingChecked::None, None, None),
            chosen_piece: None,
            dest_square: None,
            engine_move: None,
            legal_moves: Vec::new(),
            move_history: Vec::new(),
        });
    }

    pub fn reset_en_peasant_target(&mut self, initial_pos: usize, final_pos: usize) -> () {
        if let ChessPiece::P(p) = &self.squares[final_pos] {
            if p.moved_two_squares(initial_pos) {
                self.en_peasant_susceptible = match p.key.0 {
                    PieceColor::Black => Some(final_pos - 8),
                    PieceColor::White => Some(final_pos + 8),
                };
                return ();
            }
            self.en_peasant_susceptible = None;
            return ();
        }
        self.en_peasant_susceptible = None;
    }

    pub fn set(&mut self) -> GameResult {
        let mut id: u8 = 0;
        for i in 0..=7 {
            let black_pawn_idx: usize = i + 8;
            self.squares[black_pawn_idx] =
                ChessPiece::P(Pawn::new(PieceColor::Black, black_pawn_idx, id));
            self.black_locations.insert(id, black_pawn_idx);
            id += 1;

            let white_pawn_idx: usize = BOARD_AREA - black_pawn_idx - 1;
            self.squares[white_pawn_idx] =
                ChessPiece::P(Pawn::new(PieceColor::White, white_pawn_idx, id));
            self.white_locations.insert(id, white_pawn_idx);

            let white_piece_pos: usize = BOARD_AREA - i - 1;

            match i {
                0 | 7 => {
                    id += 1;
                    self.squares[i] = ChessPiece::R(Rook::new(PieceColor::Black, i, id));
                    self.black_locations.insert(id, i);
                    id += 1;
                    self.squares[white_piece_pos] =
                        ChessPiece::R(Rook::new(PieceColor::White, white_piece_pos, id));
                    self.white_locations.insert(id, white_piece_pos);
                }
                1 | 6 => {
                    id += 1;
                    self.squares[i] = ChessPiece::N(Knight::new(PieceColor::Black, i, id));
                    self.black_locations.insert(id, i);
                    id += 1;
                    self.squares[white_piece_pos] =
                        ChessPiece::N(Knight::new(PieceColor::White, white_piece_pos, id));
                    self.white_locations.insert(id, white_piece_pos);
                }
                2 | 5 => {
                    id += 1;
                    self.squares[i] = ChessPiece::B(Bishop::new(PieceColor::Black, i, id));
                    self.black_locations.insert(id, i);
                    id += 1;
                    self.squares[white_piece_pos] =
                        ChessPiece::B(Bishop::new(PieceColor::White, white_piece_pos, id));
                    self.white_locations.insert(id, white_piece_pos);
                }
                3 => {
                    id += 1;
                    self.squares[i] = ChessPiece::Q(Queen::new(PieceColor::Black, i, id));
                    self.black_locations.insert(id, i);
                    id += 1;
                    self.squares[white_piece_pos - 1] =
                        ChessPiece::Q(Queen::new(PieceColor::White, white_piece_pos - 1, id));
                    self.white_locations.insert(id, white_piece_pos - 1);
                }
                4 => {
                    id += 1;
                    self.squares[i] = ChessPiece::K(King::new(PieceColor::Black, i, id));
                    self.black_locations.insert(id, i);
                    id += 1;
                    self.squares[white_piece_pos + 1] =
                        ChessPiece::K(King::new(PieceColor::White, white_piece_pos + 1, id));
                    self.white_locations.insert(id, white_piece_pos + 1);
                }
                _ => (),
            }
        }

        return Ok(());
    }

    pub fn handle_selection(&mut self) -> GameResult {
        let selection_idx: usize = if let Some(x) = self.chosen_piece {
            x
        } else {
            return Ok(());
        };

        match &self.squares[selection_idx] {
            ChessPiece::Square(_) => (),
            piece => match (&self.gamemode, piece.color()) {
                (&GameMode::SelectionWhite, Some(PieceColor::White))
                | (&GameMode::SelectionBlack, Some(PieceColor::Black)) => {
                    self.legal_moves = piece.legal_moves(
                        &self,
                        self.en_peasant_susceptible,
                        &self.check,
                        match piece.color().unwrap() {
                            PieceColor::Black => *self.black_locations.get(&14).unwrap(),
                            PieceColor::White => *self.white_locations.get(&15).unwrap(),
                        },
                    )?;
                    piece.generate_vision(&self);
                }
                _ => return Ok(()),
            },
        };

        match (
            &self.gamemode,
            &self.squares[selection_idx].is_piece(),
            &self.squares[selection_idx],
        ) {
            (GameMode::SelectionWhite, true, piece) => {
                if piece.color() != Some(PieceColor::White) {
                    self.chosen_piece = None;
                } else {
                    self.gamemode = GameMode::MovementWhite;
                }
            }
            (GameMode::SelectionBlack, true, piece) => {
                if piece.color() != Some(PieceColor::Black) {
                    self.chosen_piece = None;
                } else {
                    self.gamemode = GameMode::MovementBlack;
                }
            }
            _ => {
                self.chosen_piece = None;
            }
        };

        return Ok(());
    }

    fn any_legal_moves(&mut self, color: PieceColor) -> bool {
        let (map, checked_king_idx) = match color {
            PieceColor::Black => (
                &self.black_locations,
                *self.black_locations.get(&14).unwrap(),
            ),
            PieceColor::White => (
                &self.white_locations,
                *self.white_locations.get(&15).unwrap(),
            ),
        };
        for (_, idx) in map {
            let legal_moves: GameResult<Vec<usize>> = self.squares[*idx].legal_moves(
                &self,
                self.en_peasant_susceptible,
                &self.check,
                checked_king_idx,
            );
            if let Ok(legal_moves_vec) = legal_moves {
                if legal_moves_vec.len() > 0 {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn perform_move(
        &mut self,
        initial_pos: usize,
        final_pos: usize,
        whose_turn: PieceColor,
    ) -> GameResult {
        self.squares[initial_pos].new_idx(final_pos);

        let mut this_move: MoveCancellation = MoveCancellation::new();
        this_move.previous_gamemode = Some(self.gamemode.clone());
        this_move.en_peasant = self.en_peasant_susceptible;

        match self.squares[initial_pos] {
            ChessPiece::R(ref mut r) => {
                r.was_moved = true;
                this_move.was_moved = true;
            }
            ChessPiece::K(ref mut k) => {
                k.was_moved = true;
                this_move.was_moved = true;
            }
            ChessPiece::P(ref mut p) => {
                p.was_moved = true;
                this_move.was_moved = true;
                if let Some(target) = self.en_peasant_susceptible {
                    if target == final_pos {
                        let enemy_pawn_idx: usize = match p.key.0 {
                            PieceColor::Black => final_pos - 8,
                            PieceColor::White => final_pos + 8,
                        };
                        self.take_piece(enemy_pawn_idx)?;
                        let _ = std::mem::replace(
                            &mut self.squares[enemy_pawn_idx],
                            ChessPiece::Square(Void),
                        );
                    }
                }
            }
            _ => (),
        };
        self.reset_en_peasant_target(initial_pos, final_pos);
        let taken_piece: ChessPiece = self.squares[final_pos];
        if taken_piece.is_piece() && taken_piece.color() != Some(whose_turn) {
            this_move.captured_piece =
                Some((taken_piece.id()?, final_pos, taken_piece.key().unwrap().1));
            self.take_piece(final_pos)?;
        }
        println!("initial: {:?}", self.squares[initial_pos]);
        self.squares[final_pos] = self.squares[initial_pos];
        self.squares[initial_pos] = ChessPiece::Square(Void);
        let moving_piece: ChessPiece = self.squares[final_pos];
        println!("final: {:?}", self.squares[final_pos]);
        this_move.moved_piece = Some((moving_piece.id()?, initial_pos));

        if let ChessPiece::P(p) = &moving_piece {
            this_move.promoted_pawn = Some((p.id, initial_pos));
            match self.gamemode {
                GameMode::MovementBlack => self.promote(p.index, "q"),
                GameMode::MovementWhite => {
                    if (0..=7).contains(&p.index) {
                        let mut promotion_choice: String = String::new();
                        loop {
                            println!(
                                "choose a piece you want your pawn to become: q (queen), r (rook), b (bishop), n (knight)"
                            );
                            stdin().read_line(&mut promotion_choice).unwrap();
                            if ["q", "r", "n", "b"]
                                .contains(&promotion_choice.trim().to_ascii_lowercase().as_str())
                            {
                                break;
                            }
                        }
                        self.promote(p.index, &promotion_choice);
                    }
                }
                _ => (),
            };
        }
        match self.squares[final_pos].color().unwrap() {
            PieceColor::Black => &mut self.black_locations,
            PieceColor::White => &mut self.white_locations,
        }
        .insert(moving_piece.id()?, final_pos);

        if let ChessPiece::K(k) = &moving_piece
            && std::cmp::max(initial_pos, final_pos) - std::cmp::min(initial_pos, final_pos) == 2
        {
            let color: PieceColor = match k.key.0 {
                c => c,
            };
            let king_rank: usize = initial_pos / 8;
            let left_rook_idx: usize = king_rank * 8 + 0;
            let right_rook_idx: usize = king_rank * 8 + 7;
            let (rook_initial_idx, rook_final_idx) = if initial_pos > final_pos {
                (left_rook_idx, final_pos + 1)
            } else {
                (right_rook_idx, final_pos - 1)
            };
            self.perform_move(rook_initial_idx, rook_final_idx, color)?;
            // Remove the auxiliary rook move from history; castling should be a single user move
            let _ = self.move_history.pop();
            {
                let map: &mut HashMap<u8, usize> = match color {
                    PieceColor::Black => &mut self.black_locations,
                    PieceColor::White => &mut self.white_locations,
                };
                map.insert(self.squares[rook_final_idx].id()?, rook_final_idx);
            }
            this_move.castled_rook = Some((
                self.squares[rook_final_idx].id()?,
                rook_initial_idx,
                rook_final_idx,
            ));
        }
        match whose_turn {
            PieceColor::Black => {
                self.black_locations
                    .insert(self.squares[final_pos].id()?, final_pos);
                self.black_vision()?;
                if !self.any_legal_moves(PieceColor::White) {
                    if self.checked == KingChecked::White {
                        self.gamemode = GameMode::BlackWin;
                    } else {
                        self.gamemode = GameMode::Draw;
                    }
                } else {
                    self.gamemode = GameMode::SelectionWhite;
                }
            }
            PieceColor::White => {
                self.white_locations
                    .insert(self.squares[final_pos].id()?, final_pos);
                self.white_vision()?;
                if !self.any_legal_moves(PieceColor::Black) {
                    if self.checked == KingChecked::Black {
                        self.gamemode = GameMode::WhiteWin;
                    } else {
                        self.gamemode = GameMode::Draw;
                    }
                } else {
                    self.gamemode = GameMode::SelectionBlack;
                }
            }
        }
        this_move.check_state = self.check;
        this_move.whose_turn = Some(whose_turn);
        self.move_history.push(this_move);
        return Ok(());
    }

    pub fn cancel_move(&mut self) -> GameResult {
        if let Some(takeback) = self.move_history.pop() {
            let (map, enemy_map) = match takeback.whose_turn.unwrap() {
                PieceColor::Black => (&mut self.black_locations, &mut self.white_locations),
                PieceColor::White => (&mut self.white_locations, &mut self.black_locations),
            };
            let moved_id: u8 = takeback.moved_piece.unwrap().0;
            let mut new_idx: usize = 0;
            for i in 0..BOARD_AREA {
                if self.squares[i].is_piece() {
                    if let Ok(id) = self.squares[i].id() {
                        if id == moved_id {
                            new_idx = i;
                            break;
                        }
                    }
                }
            }
            let old_idx: usize = takeback.moved_piece.unwrap().1;
            let piece: &mut ChessPiece = &mut self.squares[new_idx];
            piece.new_idx(old_idx);
            if takeback.was_moved {
                piece.reset_was_moved(false);
            }
            let piece: &ChessPiece = &self.squares[new_idx];
            if let Some(pawn) = takeback.promoted_pawn {
                self.squares[old_idx] = ChessPiece::P(Pawn {
                    index: old_idx,
                    value: PAWN_VALUE,
                    key: piece.key().unwrap(),
                    was_moved: true,
                    id: pawn.0,
                    is_pinned: false,
                });
            } else {
                self.squares[old_idx] = *piece;
            }

            if let Some(rook) = takeback.castled_rook {
                let castled_rook: ChessPiece = self.squares[rook.2];
                let castled_rook_id = castled_rook.id()?;
                let castled_rook_key = castled_rook.key().unwrap();
                self.squares[rook.1] = ChessPiece::R(Rook {
                    value: ROOK_VALUE,
                    key: castled_rook_key,
                    was_moved: false,
                    index: rook.1,
                    id: castled_rook_id,
                    is_pinned: false,
                });
                self.squares[rook.2] = ChessPiece::Square(Void);
                map.insert(castled_rook_id, rook.1);
            }

            if let Some(piece) = takeback.captured_piece {
                let taken_piece_color: PieceColor = match takeback.whose_turn.unwrap() {
                    PieceColor::Black => PieceColor::White,
                    PieceColor::White => PieceColor::Black,
                };
                self.squares[new_idx] = match piece.2 {
                    PieceVariant::B => {
                        ChessPiece::B(Bishop::new(taken_piece_color, piece.1, piece.0))
                    }
                    PieceVariant::N => {
                        ChessPiece::N(Knight::new(taken_piece_color, piece.1, piece.0))
                    }
                    PieceVariant::R => {
                        ChessPiece::R(Rook::new(taken_piece_color, piece.1, piece.0))
                    }
                    PieceVariant::Q => {
                        ChessPiece::Q(Queen::new(taken_piece_color, piece.1, piece.0))
                    }
                    PieceVariant::P => {
                        ChessPiece::P(Pawn::new(taken_piece_color, piece.1, piece.0))
                    }
                    _ => unreachable!(),
                };
                enemy_map.insert(piece.0, piece.1);
            } else {
                self.squares[new_idx] = ChessPiece::Square(Void);
            }

            map.insert(takeback.moved_piece.unwrap().0, old_idx);
            self.checked = takeback.check_state.0;
            self.check = takeback.check_state;
            self.en_peasant_susceptible = takeback.en_peasant;
            self.gamemode = takeback.previous_gamemode.unwrap();
        }
        return Ok(());
    }

    pub fn take_piece(&mut self, final_pos: usize) -> GameResult {
        match self.squares[final_pos].color().unwrap() {
            PieceColor::Black => self.black_locations.remove(&self.squares[final_pos].id()?),
            PieceColor::White => self.white_locations.remove(&self.squares[final_pos].id()?),
        };
        return Ok(());
    }

    pub fn white_vision(&mut self) -> GameResult {
        let mut vision: HashSet<usize> = HashSet::new();
        for idx in self.white_locations.values() {
            vision.extend(match self.squares[*idx].generate_vision(&self) {
                Some(v) => v,
                None => Vec::new(),
            });
        }
        self.white_vision = vision;
        return Ok(());
    }

    pub fn black_vision(&mut self) -> GameResult {
        let mut vision: HashSet<usize> = HashSet::new();
        for idx in self.black_locations.values() {
            vision.extend(match self.squares[*idx].generate_vision(&self) {
                Some(v) => v,
                None => Vec::new(),
            });
        }
        self.black_vision = vision;
        return Ok(());
    }

    pub fn is_check(&mut self, attacker_color: PieceColor) -> () {
        let (search_map, enemy_map, enemy_king_id, mut under_check) = match attacker_color {
            PieceColor::White => (
                &self.white_locations,
                &self.black_locations,
                14,
                KingChecked::None,
            ),
            PieceColor::Black => (
                &self.black_locations,
                &self.white_locations,
                15,
                KingChecked::None,
            ),
        };
        let mut attacker1: Option<usize> = None;
        let mut attacker2: Option<usize> = None;
        let mut attackers_counter: u8 = 0;

        for (_, index) in search_map {
            match &self.squares[*index] {
                ChessPiece::K(_) | ChessPiece::Square(_) => continue,
                other => {
                    let vision: Vec<usize> = other.generate_vision(&self).unwrap();
                    if vision.contains(&enemy_map.get(&enemy_king_id).unwrap())
                        && attackers_counter == 0
                    {
                        attacker1 = Some(other.index().unwrap());
                        attackers_counter += 1;
                        under_check = match enemy_map {
                            a if a == &self.white_locations => KingChecked::Black,
                            a if a == &self.black_locations => KingChecked::White,
                            _ => unreachable!(),
                        };
                    } else if vision.contains(&enemy_map.get(&enemy_king_id).unwrap())
                        && attackers_counter == 1
                    {
                        attacker2 = Some(other.index().unwrap());
                        self.check = (under_check, attacker1, attacker2);
                        return ();
                    }
                }
            }
        }
        self.checked = under_check;
        self.check = (under_check, attacker1, attacker2);
    }

    pub fn promote(&mut self, index: usize, choice: &str) -> () {
        if let ChessPiece::P(p) = &self.squares[index] {
            if match p.key.0 {
                PieceColor::Black => 56..=63,
                PieceColor::White => 0..=7,
            }
            .contains(&p.index)
            {
                let (color, id) = (p.key.0, p.id);
                match choice.trim().to_ascii_lowercase().as_str() {
                    "q" => {
                        let _ = std::mem::replace(
                            &mut self.squares[index],
                            ChessPiece::Q(Queen::new(color, index, id)),
                        );
                    }
                    "r" => {
                        let _ = std::mem::replace(
                            &mut self.squares[index],
                            ChessPiece::R(Rook::new(color, index, id)),
                        );
                    }
                    "b" => {
                        let _ = std::mem::replace(
                            &mut self.squares[index],
                            ChessPiece::B(Bishop::new(color, index, id)),
                        );
                    }
                    "n" => {
                        let _ = std::mem::replace(
                            &mut self.squares[index],
                            ChessPiece::N(Knight::new(color, index, id)),
                        );
                    }
                    _ => println!("one of the four letters must be entered: q, r, b or n"),
                };
            }
        }
    }
}

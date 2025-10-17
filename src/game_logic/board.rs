use ggez::GameResult;
use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use crate::{
    constants::BOARD_AREA,
    game_logic::{
        PieceColor,
        pieces::{Bishop, ChessPiece, King, Knight, Pawn, Queen, Rook, Void},
        state_enums::{GameMode, KingChecked},
    },
    helper_functions::generate_empty_board,
};

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
        en_peasant_target: Option<usize>,
        whose_turn: PieceColor,
    ) -> GameResult {
        self.squares[initial_pos].new_idx(final_pos);
        match self.squares[initial_pos] {
            ChessPiece::R(ref mut r) => r.was_moved = true,
            ChessPiece::K(ref mut k) => k.was_moved = true,
            ChessPiece::P(ref mut p) => {
                p.was_moved = true;
                if let Some(target) = en_peasant_target {
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
        if self.squares[final_pos].is_piece() && self.squares[final_pos].color() != Some(whose_turn)
        {
            self.take_piece(final_pos)?;
        }
        let moving_piece: ChessPiece =
            std::mem::replace(&mut self.squares[initial_pos], ChessPiece::Square(Void));

        let _ = std::mem::replace(&mut self.squares[final_pos], moving_piece);
        if let ChessPiece::P(p) = &self.squares[final_pos] {
            match self.gamemode {
                GameMode::MovementBlack => self.promote(p.index, "q"),
                GameMode::MovementWhite => {
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
                _ => unreachable!(),
            }
        }
        match self.squares[final_pos].color().unwrap() {
            PieceColor::Black => &mut self.black_locations,
            PieceColor::White => &mut self.white_locations,
        }
        .insert(moving_piece.id().unwrap(), final_pos);
        if let ChessPiece::K(k) = &self.squares[final_pos]
            && std::cmp::max(initial_pos, final_pos) - std::cmp::min(initial_pos, final_pos) == 2
        {
            match k.key.0 {
                PieceColor::Black => {
                    if initial_pos > final_pos {
                        self.perform_move(0, final_pos + 1, en_peasant_target, PieceColor::Black)?;
                        self.black_locations
                            .insert(self.squares[final_pos + 1].id()?, final_pos + 1);
                    } else {
                        self.perform_move(7, final_pos - 1, en_peasant_target, PieceColor::Black)?;
                        self.black_locations
                            .insert(self.squares[final_pos - 1].id()?, final_pos - 1);
                    }
                }
                PieceColor::White => {
                    if initial_pos > final_pos {
                        self.perform_move(56, final_pos + 1, en_peasant_target, PieceColor::White)?;
                        self.white_locations
                            .insert(self.squares[final_pos + 1].id()?, final_pos + 1);
                    } else {
                        self.perform_move(63, final_pos - 1, en_peasant_target, PieceColor::White)?;
                        self.white_locations
                            .insert(self.squares[final_pos - 1].id()?, final_pos - 1);
                    }
                }
            }
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

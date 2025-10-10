use ggez::{
    Context, GameResult,
    graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
};
use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use crate::{
    constants::{BOARD_AREA, COORDS, SQUARE_SIDE},
    game_logic::{
        PieceColor,
        pieces::{Bishop, ChessPiece, King, Knight, Pawn, Queen, Rook},
        state_enums::KingChecked,
    },
    helper_functions::generate_empty_board,
};

pub struct Board {
    pub squares: [ChessPiece; BOARD_AREA],
    pub board_mesh: Mesh,
    pub white_locations: HashMap<u8, usize>,
    pub black_locations: HashMap<u8, usize>,
    pub white_vision: HashSet<usize>,
    pub black_vision: HashSet<usize>,
    pub checked: KingChecked,
}
impl Board {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut mesh_builder: MeshBuilder = MeshBuilder::new();
        for (x, y) in COORDS {
            mesh_builder.rectangle(
                DrawMode::fill(),
                Rect::new(
                    x as f32 * SQUARE_SIDE,
                    y as f32 * SQUARE_SIDE,
                    SQUARE_SIDE as f32,
                    SQUARE_SIDE as f32,
                ),
                match x % 2 == y % 2 {
                    true => Color::from_rgb(250, 230, 250),
                    false => Color::from_rgb(70, 50, 130),
                },
            )?;
        }
        return Ok(Board {
            squares: generate_empty_board(),
            board_mesh: mesh_builder.build(ctx)?,
            white_locations: HashMap::new(),
            black_locations: HashMap::new(),
            white_vision: HashSet::new(),
            black_vision: HashSet::new(),
            checked: KingChecked::None,
        });
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

    pub fn is_check(
        &mut self,
        attacker_color: PieceColor,
    ) -> (KingChecked, Option<usize>, Option<usize>) {
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
                        return (under_check, attacker1, attacker2);
                    }
                }
            }
        }
        self.checked = under_check;
        return (under_check, attacker1, attacker2);
    }

    pub fn promote(&mut self, index: usize) -> () {
        if let ChessPiece::P(p) = &self.squares[index] {
            if match p.key.0 {
                PieceColor::Black => 56..=63,
                PieceColor::White => 0..=7,
            }
            .contains(&p.index)
            {
                let (color, id) = (p.key.0, p.id);
                loop {
                    println!(
                        "choose a piece you want your pawn to become: q (queen), r (rook), b (bishop), n (knight)"
                    );
                    let mut promotion_choice: String = String::new();
                    stdin().read_line(&mut promotion_choice).unwrap();
                    match promotion_choice.trim().to_ascii_lowercase().as_str() {
                        "q" => {
                            let _ = std::mem::replace(
                                &mut self.squares[index],
                                ChessPiece::Q(Queen::new(color, index, id)),
                            );
                            break;
                        }
                        "r" => {
                            let _ = std::mem::replace(
                                &mut self.squares[index],
                                ChessPiece::R(Rook::new(color, index, id)),
                            );
                            break;
                        }
                        "b" => {
                            let _ = std::mem::replace(
                                &mut self.squares[index],
                                ChessPiece::B(Bishop::new(color, index, id)),
                            );
                            break;
                        }
                        "n" => {
                            let _ = std::mem::replace(
                                &mut self.squares[index],
                                ChessPiece::N(Knight::new(color, index, id)),
                            );
                            break;
                        }
                        _ => println!("one of the four letters must be entered: q, r, b or n"),
                    };
                }
            }
        }
    }
}

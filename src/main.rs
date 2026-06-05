#[cfg(feature = "debug-ui")]
use crate::{
    board_geometry_templates::{MARK_SHIFT, TO_SHIFT},
    converters::fen_converter::fen_to_board,
};
use crate::{
    constants::attacks::{
        compute_all_lines, compute_all_rays, compute_all_rays_from, compute_mvvlva,
        initialize_sliding_attack_tables,
    },
    search::Engine,
    uci::uci_output,
};
pub mod board;
pub mod board_geometry_templates;
pub mod constants;
pub mod converters;
pub mod enums;
pub mod gamestate;
pub mod move_generation;
pub mod move_make_unmake;
pub mod moves;
pub mod search;
pub mod tests;
pub mod transposition;
pub mod tuning;
pub mod uci;

#[cfg(not(feature = "debug-ui"))]
fn main() -> () {
    /* initialize_sliding_attack_tables(), compute_all_rays(),
    compute_all_lines, compute_mvvlva
    and compute_all_rays_from() have to be called
    in the beginning of program and tests */
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let mut engine: Engine = Engine::new(8, 20);
    uci_output(&mut engine);
    std::process::exit(0);
}

#[cfg(feature = "debug-ui")]
mod visual_debugger;
#[cfg(feature = "debug-ui")]
use {
    crate::{board_highlight::*, board_visual::*, visual_debugger::*},
    macroquad::prelude::*,
};
#[cfg(feature = "debug-ui")]
#[macroquad::main("Ferrous debugger")]
async fn main() {
    println!("hello, debug-ui");
    set_fullscreen(true);

    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let pieces_images: [Texture2D; 12] = [
        load_texture("pieces/white_pawn.png").await.unwrap(),
        load_texture("pieces/white_knight.png").await.unwrap(),
        load_texture("pieces/white_bishop.png").await.unwrap(),
        load_texture("pieces/white_rook.png").await.unwrap(),
        load_texture("pieces/white_queen.png").await.unwrap(),
        load_texture("pieces/white_king.png").await.unwrap(),
        load_texture("pieces/black_pawn.png").await.unwrap(),
        load_texture("pieces/black_knight.png").await.unwrap(),
        load_texture("pieces/black_bishop.png").await.unwrap(),
        load_texture("pieces/black_rook.png").await.unwrap(),
        load_texture("pieces/black_queen.png").await.unwrap(),
        load_texture("pieces/black_king.png").await.unwrap(),
    ];
    for texture in &pieces_images {
        texture.set_filter(FilterMode::Nearest);
    }

    let (mut board, mut state) =
        fen_to_board("r4r2/2p2p1k/5p1p/2p1p3/6PN/P1P4P/KP2qP2/7R b - - 3 4");
    board.total_occupancy();
    board.update_full_cache();
    board.perform_move(56 | (16 << TO_SHIFT), &mut state, 16, &mut 0, &mut 0);

    let mut debugger_board = board_visual::BoardVisual {
        squares: [0; 64],
        normal_moves: Vec::new(),
        captures: Vec::new(),
        en_passant: (0, 64),
        castlings: Vec::new(),
        selected_square: 64,
    };
    debugger_board.set_pieces(&board, &state);
    debugger_board.get_moves(&board, &state, 8);

    let mut should_highlight: bool = false;

    loop {
        clear_background(GRAY);
        board_visual::draw_board();
        if should_highlight {
            debugger_board.highlight_legal_moves();
        }
        debugger_board.draw_pieces(&pieces_images);

        if is_mouse_button_pressed(MouseButton::Left) {
            if !should_highlight {
                let (mouse_x, mouse_y) = mouse_position();
                debugger_board.selected_square = calculate_index(mouse_x as u16, mouse_y as u16);
                should_highlight = true;
            } else {
                debugger_board.selected_square = 64;
                should_highlight = false;
            }
        }
        next_frame().await;
    }
}

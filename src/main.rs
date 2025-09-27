pub mod constants;
pub mod game_logic;
pub mod helper_functions;
use constants::WIN_SCALES;
use game_logic::{GameResult, MainState};

use ggez::{ContextBuilder, conf::WindowMode, event, graphics};
use std::env::current_dir;

fn main() -> GameResult {
    let (mut ctx, mainloop) = ContextBuilder::new("Chess", "Sava2008")
        .window_mode(WindowMode::default().dimensions(WIN_SCALES.0, WIN_SCALES.1))
        .add_resource_path(current_dir()?.join("Pieces"))
        .build()?;
    let mut state: MainState = MainState::new(&mut ctx)?;
    state.board.set()?;
    graphics::set_window_title(&ctx, "Chess");
    event::run(ctx, mainloop, state);
}

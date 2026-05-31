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

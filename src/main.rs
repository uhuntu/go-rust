extern crate ggez;
#[macro_use]
extern crate lazy_static;

mod draw;
mod game;
mod pixel_math;

use game::MainState;

use ggez::conf;
use ggez::event;
use ggez::{ContextBuilder, GameResult};

fn main() -> GameResult {
    let cb = ContextBuilder::new("Go", "ggez")
        .window_setup(conf::WindowSetup::default().title("Go: The Way of the Warrior"))
        .window_mode(conf::WindowMode::default().dimensions(
            pixel_math::SCREEN_SIZE.0 as f32,
            pixel_math::SCREEN_SIZE.1 as f32,
        ));

    let (mut ctx, event_loop) = cb.build().unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    // if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    //     let mut path = path::PathBuf::from(manifest_dir);
    //     path.push("resources");
    //     ctx.filesystem.mount(&path, true);
    // }

    // println!("{}", graphics::get_renderer_info(ctx).unwrap());

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state)
}

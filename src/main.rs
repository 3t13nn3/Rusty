//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod background;

use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics;
use ggez::GameResult;
use glam::*;

use std::env;
use std::path;

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("^^", "Etienne")
        .add_resource_path(resource_dir)
        .window_mode(WindowMode {
            width: WIDTH,
            height: HEIGHT,
            ..Default::default()
        });
    let (mut ctx, event_loop) = cb.build()?;
    graphics::set_window_title(&ctx, "Etienne on Rust");

    //Adding background thing
    let state = background::Backgrounds::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

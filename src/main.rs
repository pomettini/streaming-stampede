extern crate ggez;
extern crate sdl2;

use ggez::*;
use std::{env, path};

// Using custom version of event
pub mod event;

pub mod constants;
pub mod pokemon_types;
pub mod pokemons;
pub mod player;
pub mod states;
pub mod ui;
pub mod assets;
pub mod utils;
pub mod scenes;

use constants::*;
use pokemon_types::*;
use pokemons::*;
use player::*;
use states::*;
use ui::*;
use assets::*;
use utils::*;
use scenes::*;

pub fn main() 
{
    let mut cb = ContextBuilder::new("Streaming Stampede", "Pomettini")
        .window_setup(conf::WindowSetup::default().title("Streaming Stampede"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") 
    {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources/");
        cb = cb.add_resource_path(path);
    } 

    let ctx = &mut cb.build().unwrap();

    event::run(ctx);
}
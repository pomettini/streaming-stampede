use ggez::graphics::{Point};

pub const WINDOW_W: u32 = 1024;
pub const WINDOW_H: u32 = 768;

#[derive(PartialEq)]
pub enum GameState
{
    Start,
    Tutorial,
    Countdown,
    Question,
    Race,
    Answer,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum PokemonType
{
    Slugma,
    Magcargo,
    Diglett,
    Dugtrio,
    Voltorb,
    Electrode,
    Pichu,
    Pikachu,
    Togepi,
    Doduo,
    // Dodrio,
    // Psyduck,
    // Omanyte,
    // Magnemite,
    // Magneton,
    // Pokeball,
    // Egg,
}

pub struct Pokemon
{
    pub ptype: PokemonType,
    pub pos: Point,
    pub spr_frame: f32,
    pub spr_num_frames: u8,
    pub speed: f32,
    pub isfake: bool,
}
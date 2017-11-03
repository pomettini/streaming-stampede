extern crate ggez;
extern crate rand;

use rand::Rng;

use ggez::conf;
use ggez::event;
use ggez::event::*;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Point};
use ggez::timer;
use std::time::Duration;
use ggez::audio;

const WINDOW_W: u32 = 768;
const WINDOW_H: u32 = 768;

const GAME_STATES_MAX: u32 = 4;
const GAME_STATE_PLAY: u32 = GAME_STATES_MAX - 1;

enum PokemonType
{
    None,
    Slugma,
    Magcargo,
    // Diglett,
    // Dugtrio,
    // Voltorb,
    // Electrode,
    // Pichu,
    // Pikachu,
    // Togepi,
    // Doduo,
    // Dodrio,
    // Psyduck,
    // Omanyte,
    // Magnemite,
    // Magneton,
    // Pokeball,
    // Egg,
}

struct Vector2
{
    x: f32,
    y: f32,
}

struct Pokemon
{
    ptype: PokemonType,
    sprite: Vec<graphics::Image>,
    sprite_num: u8,
    pos: Vector2,
    speed: f32,
    isfake: bool,
}

impl Pokemon
{
    fn new(ctx: &mut Context) -> Pokemon 
    {
        let y_pos = rand::thread_rng().gen_range(0, WINDOW_H);
        let speed_boost = rand::thread_rng().gen_range(-50, -25);
        let x_pos = rand::thread_rng().gen_range(10, 1000);

        Pokemon 
        {
            ptype: PokemonType::Slugma,
            sprite: vec![
                graphics::Image::new(ctx, "/slugma0.png").unwrap(),
                graphics::Image::new(ctx, "/slugma1.png").unwrap(),
                graphics::Image::new(ctx, "/slugma2.png").unwrap(),
                graphics::Image::new(ctx, "/slugma3.png").unwrap(),
                graphics::Image::new(ctx, "/slugma4.png").unwrap(),
                graphics::Image::new(ctx, "/slugma5.png").unwrap(),
            ],
            sprite_num: 0,
            pos: Vector2 {x: WINDOW_H as f32 + x_pos as f32, y: y_pos as f32},
            speed: -speed_boost as f32 / 10.0,
            isfake: false,
        }
    }

    pub fn update(&mut self) 
    {
        self.pos.x -= self.speed;
        self.sprite_num += 1;

        if(self.sprite_num >= self.sprite.len() as u8)
        {
            self.sprite_num = 0;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> 
    {
        let dest_point = graphics::Point::new(self.pos.x, self.pos.y);
        graphics::draw(ctx, &self.sprite[(self.sprite_num) as usize], dest_point, 0.0)?;

        Ok(())
    }
}

struct MainState 
{
    pokemon: Vec<Pokemon>,
    font: graphics::Font,
    title: graphics::Text,
    state: u32,
}

impl MainState 
{
    fn new(ctx: &mut Context) -> GameResult<MainState> 
    {
        let mut pokemon = vec![];
        let font = graphics::Font::new(ctx, "/pacifico.ttf", 80)?;
        let title = graphics::Text::new(ctx, "Streaming Stampede", &font)?;

        for _ in 0..30 
        {
            pokemon.push(Pokemon::new(ctx));
        }

        let s = MainState 
        { 
            pokemon: pokemon,
            font: font,
            title: title,
            state: 0,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState 
{
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> 
    {
        // Calling the update method for every Pokemon
        for id in 0..30 
        {
            self.pokemon[id].update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> 
    {
        graphics::clear(ctx);
        
        if self.state == 0 || self.state > GAME_STATE_PLAY 
        {
            let text_pos_x: f32 = WINDOW_W as f32 / 2.0;
            let text_pos_y: f32 = WINDOW_H as f32 / 4.0;
            graphics::draw(ctx, &self.title, Point { x: text_pos_x, y: text_pos_y }, 0.0)?;
        }

        if self.state < GAME_STATE_PLAY 
        {
            for id in 0..30 
            {
                self.pokemon[id].draw(ctx);
            }
        }

        graphics::present(ctx);
        Ok(())
    }
}


fn main() 
{
    let mut c = conf::Conf::new();
    c.window_title = "Streaming Stampede".to_string();
    c.window_width = WINDOW_W;
    c.window_height = WINDOW_H;

    let ctx = &mut Context::load_from_conf("streaming-stampede", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) 
    {
        println!("Error encountered: {}", e);
    } 
    else 
    {
        println!("Game exited cleanly.");
    }
}
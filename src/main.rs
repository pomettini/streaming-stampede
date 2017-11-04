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

struct Pokemon
{
    ptype: PokemonType,
    pos: Point,
    speed: f32,
    isfake: bool,
}

// impl Pokemon
// {
//     fn new(ctx: &mut Context) -> Pokemon 
//     {
//         let y_pos = rand::thread_rng().gen_range(0, WINDOW_H);
//         let speed_boost = rand::thread_rng().gen_range(-50, -25);
//         let x_pos = rand::thread_rng().gen_range(10, 1000);

//         Pokemon 
//         {
//             ptype: PokemonType::Slugma,
//             sprite_num: 0,
//             speed: -speed_boost as f32 / 10.0,
//             isfake: false,
//         }
//     }

//     pub fn update(&mut self) 
//     {
//         self.pos.x -= self.speed;
//         self.sprite_num += 1;

//         if(self.sprite_num >= self.sprite.len() as u8)
//         {
//             self.sprite_num = 0;
//         }

//         if(self.pos.x < 0.)
//         {
//             self.pos.x = WINDOW_H as f32;
//         }
//     }
// }
fn spawn_pokemon(ptype: PokemonType) -> Pokemon
{
    let mut speed = 0;
    let mut isfake = false;

    match ptype
    {
        PokemonType::Slugma => { speed = 1; isfake = false },
        PokemonType::Magcargo => { speed = 1; isfake = false },
    }

    Pokemon 
    {
        ptype: ptype,
        pos: Point{x: 0., y: 0.},
        speed: speed as f32,
        isfake: isfake,
    }
}

struct MainState 
{
    assets: Assets,
    pokemon: Vec<Pokemon>,
    font: graphics::Font,
    title: graphics::Text,
    state: u32,
}

impl MainState 
{
    fn new(ctx: &mut Context) -> GameResult<MainState> 
    {
        ctx.print_resource_stats();
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        let assets = Assets::new(ctx)?;
        let mut pokemon = vec![];
        let font = graphics::Font::new(ctx, "/pacifico.ttf", 80)?;
        let title = graphics::Text::new(ctx, "Streaming Stampede", &font)?;

        // for _ in 0..30 
        // {
        //     pokemon.push(Pokemon::new(ctx));
        // }

        pokemon.push(spawn_pokemon(PokemonType::Slugma));

        let s = MainState 
        { 
            assets: assets,
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
        // for p in &mut self.pokemon 
        // {
        //     p.update();
        // }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> 
    {
        // Clearing the screen
        graphics::clear(ctx);
        
        // if self.state == 0
        // {
        //     let text_pos_x: f32 = WINDOW_W as f32 / 2.0;
        //     let text_pos_y: f32 = WINDOW_H as f32 / 4.0;

        //     graphics::draw(ctx, &self.title, Point { x: text_pos_x, y: text_pos_y }, 0.0)?;
        // }

        // if self.state < GAME_STATE_PLAY 
        // {
        //     for id in 0..30 
        //     {
        //         self.pokemon[id].draw(ctx);
        //     }
        // }

        let assets = &mut self.assets;

        for p in &self.pokemon 
        {
            draw_pokemon(assets, ctx, p)?;
        }

        // Flipping the screen
        graphics::present(ctx);

        Ok(())
    }
}

struct Assets 
{
    slugma_spr: Vec<graphics::Image>,
}

impl Assets 
{
    fn new(ctx: &mut Context) -> GameResult<Assets> 
    {
        let slugma_spr = vec!
        [
            graphics::Image::new(ctx, "/slugma0.png").unwrap(),
            graphics::Image::new(ctx, "/slugma1.png").unwrap(),
            graphics::Image::new(ctx, "/slugma2.png").unwrap(),
            graphics::Image::new(ctx, "/slugma3.png").unwrap(),
            graphics::Image::new(ctx, "/slugma4.png").unwrap(),
            graphics::Image::new(ctx, "/slugma5.png").unwrap(),
        ];

        Ok(Assets 
        {
            slugma_spr: slugma_spr,
        })
    }

    fn pokemon_sprite(&mut self, pokemon: &Pokemon) -> &mut Vec<graphics::Image> 
    {
        match pokemon.ptype 
        {
            PokemonType::Slugma => &mut self.slugma_spr,
            PokemonType::Magcargo => &mut self.slugma_spr,
        }
    }
}

fn draw_pokemon(assets: &mut Assets, ctx: &mut Context, pokemon: &Pokemon) -> GameResult<()> 
{
    // TODO: I have to implement a method to go from local to world coordinates to not go mad
    // And one from world to local too
    let image = assets.pokemon_sprite(pokemon);
    let pos = pokemon.pos;

    graphics::draw(ctx, &image[0], pos, 0.0)
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
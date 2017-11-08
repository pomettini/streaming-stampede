extern crate ggez;
extern crate rand;

use rand::Rng;

use ggez::conf;
use ggez::event;
use ggez::event::*;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Point};
use ggez::graphics::{Rect};
use ggez::timer;
use std::time::Duration;
use ggez::audio;

const WINDOW_W: u32 = 1024;
const WINDOW_H: u32 = 768;

const GAME_STATES_MAX: u32 = 4;
const GAME_STATE_PLAY: u32 = GAME_STATES_MAX - 1;

enum PokemonType
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

struct Pokemon
{
    ptype: PokemonType,
    pos: Point,
    spr_frame: f32,
    spr_num_frames: u8,
    speed: f32,
    isfake: bool,
}

impl Pokemon
{
    fn new(ctx: &mut Context) -> Pokemon 
    {
        Pokemon 
        {
            ptype: PokemonType::Slugma,
            pos: Point{ x: 0., y: 0. },
            spr_frame: 0.,
            spr_num_frames: 0,
            speed: 0.,
            isfake: false,
        }
    }

    pub fn update(&mut self) 
    {
        self.pos.x -= self.speed;
        // self.spr_frame += 0.03 * self.speed;
        self.spr_frame += 0.25;

        // Move to the next sprite
        if(self.spr_frame as u8 >= self.spr_num_frames)
        {
            self.spr_frame = 0.;
        }

        // if(self.pos.x < 0.)
        // {
        //     self.pos.x = WINDOW_H as f32;
        // }
    }
}

fn spawn_pokemon(ptype: PokemonType) -> Pokemon
{
    let mut speed = 0;
    let mut spr_num_frames = 0;
    let mut isfake = false;

    let y_pos = rand::thread_rng().gen_range(WINDOW_H as f32 * 0.6, WINDOW_H as f32 * 0.9);
    let x_pos = rand::thread_rng().gen_range(10.0, 10000.0) + WINDOW_W as f32;
    let speed_boost = rand::thread_rng().gen_range(-6.0, -3.0);

    match ptype
    {
        PokemonType::Slugma => 
        { 
            speed = 1; 
            spr_num_frames = 6; 
            isfake = false 
        },

        PokemonType::Magcargo => 
        { 
            speed = 2; 
            spr_num_frames = 4;
            isfake = false 
        },

        PokemonType::Diglett => 
        { 
            speed = 0; 
            spr_num_frames = 5; 
            isfake = false 
        },

        PokemonType::Dugtrio => 
        { 
            speed = 0; 
            spr_num_frames = 5; 
            isfake = false 
        },

        PokemonType::Voltorb => 
        { 
            speed = 2; 
            spr_num_frames = 4; 
            isfake = false 
        },

        PokemonType::Electrode => 
        { 
            speed = 2; 
            spr_num_frames = 4; 
            isfake = false 
        },

        PokemonType::Pichu => 
        { 
            speed = 2; 
            spr_num_frames = 6; 
            isfake = false 
        },

        PokemonType::Pikachu => 
        { 
            speed = 3; 
            spr_num_frames = 6; 
            isfake = false 
        },

        PokemonType::Togepi => 
        { 
            speed = 1; 
            spr_num_frames = 6; 
            isfake = false 
        },

        PokemonType::Doduo => 
        { 
            speed = 4; 
            spr_num_frames = 6; 
            isfake = false 
        },
    }

    Pokemon 
    {
        ptype: ptype,
        pos: Point{ x: x_pos as f32, y: y_pos as f32 },
        spr_frame: 0.,
        spr_num_frames: spr_num_frames,
        speed: -speed_boost + (speed as f32 * 2.0),
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

        for _ in 0..5 
        {
            pokemon.push(spawn_pokemon(PokemonType::Slugma));
            pokemon.push(spawn_pokemon(PokemonType::Magcargo));
            pokemon.push(spawn_pokemon(PokemonType::Voltorb));
            pokemon.push(spawn_pokemon(PokemonType::Electrode));
            pokemon.push(spawn_pokemon(PokemonType::Pichu));
            pokemon.push(spawn_pokemon(PokemonType::Pikachu));
            pokemon.push(spawn_pokemon(PokemonType::Togepi));
            pokemon.push(spawn_pokemon(PokemonType::Doduo));
        }

        // pokemon.push(spawn_pokemon(PokemonType::Diglett));
        // pokemon.push(spawn_pokemon(PokemonType::Dugtrio));

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
    // Move stuff
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> 
    {
        // Moving every Pokemon
        for p in &mut self.pokemon 
        {
            p.update();
        }

        Ok(())
    }

    // Draw stuff
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> 
    {
        let assets = &mut self.assets;

        // Clearing the screen
        graphics::clear(ctx);

        let bg_width = assets.field_bg.width();
        let bg_height = assets.field_bg.height();
        
        // I draw the background
        let bg_draw_options = graphics::DrawParam
        { 
            src: Rect { x: -0.0, y: 0.0, w: 1.0, h: 1.0 },
            dest: Point 
            { 
                x: WINDOW_W as f32 / 2.0, 
                y: WINDOW_H as f32 / 2.0 
            },
            rotation: 0.0,
            scale: Point 
            { 
                x: WINDOW_W as f32 / bg_width as f32, 
                y: WINDOW_H as f32 / bg_height as f32, 
            },
            offset: Point { x: 0.0, y: 0.0 },
            shear: Point { x: 0.0, y: 0.0 },
        };
        graphics::draw_ex(ctx, &assets.field_bg, bg_draw_options);
        
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

        for p in &self.pokemon 
        {
            draw_pokemon(assets, ctx, p)?;
        }

        // Blitting! \m/
        graphics::present(ctx);

        Ok(())
    }
}

fn load_sprites(ctx: &mut Context, name: &str, frames: u8) -> Vec<graphics::Image>
{
    let mut sprites = vec![];

    for s in 0..frames
    {
        // Are you loading the correct sprites, right?
        //println!("Loading {}{}.png", name, s);

        let sprite = format!("/{}{}.png", name, s);
        sprites.push(graphics::Image::new(ctx, sprite).unwrap());
    }

    sprites
}

struct Assets 
{
    field_bg: graphics::Image,
    slugma_spr: Vec<graphics::Image>,
    magcargo_spr: Vec<graphics::Image>,
    diglett_spr: Vec<graphics::Image>,
    dugtrio_spr: Vec<graphics::Image>,
    voltorb_spr: Vec<graphics::Image>,
    electrode_spr: Vec<graphics::Image>,
    pichu_spr: Vec<graphics::Image>,
    pikachu_spr: Vec<graphics::Image>,
    togepi_spr: Vec<graphics::Image>,
    doduo_spr: Vec<graphics::Image>,
}

impl Assets 
{
    fn new(ctx: &mut Context) -> GameResult<Assets> 
    {
        Ok(Assets 
        {
            field_bg: graphics::Image::new(ctx, "/field_bg.png").unwrap(),
            slugma_spr: load_sprites(ctx, "slugma", 6),
            magcargo_spr: load_sprites(ctx, "magcargo", 4),
            diglett_spr: load_sprites(ctx, "diglett", 5),
            dugtrio_spr: load_sprites(ctx, "dugtrio", 5),
            voltorb_spr: load_sprites(ctx, "voltorb", 4),
            electrode_spr: load_sprites(ctx, "electrode", 4),
            pichu_spr: load_sprites(ctx, "pichu", 6),
            pikachu_spr: load_sprites(ctx, "pikachu", 6),
            togepi_spr: load_sprites(ctx, "togepi", 6),
            doduo_spr: load_sprites(ctx, "doduo", 6),
        })
    }

    fn pokemon_sprite(&mut self, pokemon: &Pokemon) -> &mut Vec<graphics::Image> 
    {
        match pokemon.ptype 
        {
            PokemonType::Slugma => &mut self.slugma_spr,
            PokemonType::Magcargo => &mut self.magcargo_spr,
            PokemonType::Diglett => &mut self.diglett_spr,
            PokemonType::Dugtrio => &mut self.dugtrio_spr,
            PokemonType::Voltorb => &mut self.voltorb_spr,
            PokemonType::Electrode => &mut self.electrode_spr,
            PokemonType::Pichu => &mut self.pichu_spr,
            PokemonType::Pikachu => &mut self.pikachu_spr,
            PokemonType::Togepi => &mut self.togepi_spr,
            PokemonType::Doduo => &mut self.doduo_spr,
        }
    }
}

fn draw_pokemon(assets: &mut Assets, ctx: &mut Context, pokemon: &Pokemon) -> GameResult<()> 
{
    let image = assets.pokemon_sprite(pokemon);
    let pos = pokemon.pos;
    let frame = pokemon.spr_frame;

    // I draw the background
    let sprite_options = graphics::DrawParam
    { 
        src: Rect { x: -0.0, y: 0.0, w: 1.0, h: 1.0 },
        dest: pos,
        rotation: 0.0,
        scale: Point 
        { 
            x: 2.0, 
            y: 2.0, 
        },
        offset: Point { x: 0.0, y: 0.0 },
        shear: Point { x: 0.0, y: 0.0 },
    };

    graphics::draw_ex(ctx, &image[frame as usize], sprite_options)
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
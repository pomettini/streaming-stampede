extern crate ggez;
use ggez::*;
use ggez::graphics::*;

pub mod constants;
pub mod pokemon_types;
pub mod pokemons;

use constants::*;
use pokemon_types::*;
use pokemons::*;

struct MainState 
{
    pokemons: Vec<Pokemon>,
}

impl MainState 
{
    fn new(ctx: &mut Context) -> GameResult<MainState> 
    {
        let mut pokemons = vec![];

        pokemons.push(Pokemon::new(ctx, PokemonType::Slugma));
        pokemons.push(Pokemon::new(ctx, PokemonType::Magcargo));

        let s = MainState 
        { 
            pokemons: pokemons
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState 
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> 
    {
        for p in &mut self.pokemons
        {
            p.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> 
    {
        graphics::clear(ctx);

        for p in &self.pokemons
        {
            p.draw(ctx);
        }

        graphics::present(ctx);

        Ok(())
    }
}

pub fn main() 
{
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Streaming Stampede", "Pomettini", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
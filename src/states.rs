use assets::*;
use ggez::event::*;
use ggez::graphics;
use ggez::{Context, GameResult};
use player::*;
use pokemons::*;
use ui::*;
use constants::*;

pub struct MainState {
    pub pokemons: Vec<Pokemon>,
    pub players: Vec<Player>,
    pub assets: Assets,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut pokemons = vec![];

        pokemons.push(Pokemon::new(ctx, PokemonType::Slugma));
        pokemons.push(Pokemon::new(ctx, PokemonType::Magcargo));

        let s = MainState {
            pokemons,
            players: create_players(),
            assets: Assets::new(ctx),
        };

        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for p in &mut self.pokemons {
            p.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        draw_background(ctx, &self.assets).unwrap();

        for p in &self.pokemons {
            p.draw(ctx).unwrap();
        }

        draw_players_counter(ctx, &self.players, &self.assets);

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        for player in &mut self.players {
            if keycode == *player.get_key() {
                player.increase_counter();
            }
        }
    }
}

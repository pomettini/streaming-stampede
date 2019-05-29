use constants::*;

use ggez::graphics::*;
use ggez::*;
use pokemon_sprite::*;
use pokemon_types::*;
pub struct Pokemon {
    pub pokemon_type: PokemonType,
    pub sprite: PokemonSprite,
    pub position: Point2,
    pub speed: f32,
    pub isfake: bool,
}

impl Pokemon {
    pub fn new(ctx: &mut Context, pokemon_type: PokemonType) -> Pokemon {
        setup_pokemon(ctx, pokemon_type)
    }

    pub fn update(&mut self) {
        self.position.x += self.speed;
        self.position.y += self.speed;
        self.sprite.advance_sprite();
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let sprite_options = graphics::DrawParam {
            src: Rect {
                x: -0.0,
                y: 0.0,
                w: 1.0,
                h: 1.0,
            },
            dest: self.position,
            rotation: 0.0,
            scale: Point2::new(1.0, 1.0),
            offset: Point2::new(0.0, 0.0),
            shear: Point2::new(0.0, 0.0),
            color: None,
        };

        // println!("{}", self.position);

        graphics::draw_ex(ctx, self.sprite.get_current_sprite(), sprite_options)
    }
}
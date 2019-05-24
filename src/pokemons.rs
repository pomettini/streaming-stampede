use ggez::graphics::*;
use ggez::*;
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
        // self.position.x += self.speed;
        // self.position.y += self.speed;
        // self.sprite.advance_sprite();
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
pub struct PokemonSprite {
    pub sprites: Vec<graphics::Image>,
    pub sprite_index: usize,
    pub sprite_count: usize,
}

impl PokemonSprite {
    pub fn new() -> PokemonSprite {
        PokemonSprite {
            sprites: Vec::new(),
            sprite_index: 0,
            sprite_count: 0,
        }
    }

    pub fn load_sprites(&mut self, ctx: &mut Context, name: &str, count: usize) {
        let mut sprites = vec![];

        for s in 0..count {
            // println!("Loading {}{}.png", name, s);
            let sprite = format!("/{}{}.png", name, s);
            sprites.push(graphics::Image::new(ctx, sprite).unwrap());
        }

        self.sprites = sprites;
    }

    pub fn get_current_sprite(&self) -> &graphics::Image {
        &self.sprites[self.sprite_index]
    }

    pub fn advance_sprite(&mut self) {
        if self.sprite_index < self.sprite_count - 1 {
            self.sprite_index += 1;
        } else {
            self.sprite_index = 0;
        }
    }
}

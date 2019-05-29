use ggez::*;
use ggez::graphics::*;
use ggez::Context;

#[derive(Default)]
pub struct PokemonSprite {
    pub sprites: Vec<Image>,
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

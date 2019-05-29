use ggez::graphics::*;
use ggez::Context;
use pokemons::*;
use constants::*;
use pokemon_sprite::*;

pub fn setup_pokemon(ctx: &mut Context, pokemon_type: PokemonType) -> Pokemon {
    let mut sprite = PokemonSprite::new();
    let mut spritesheet_name = "";
    let mut spritesheet_frames = 0;
    let mut speed = 0.0;
    let mut isfake = false;

    match pokemon_type {
        PokemonType::Slugma => {
            spritesheet_name = "slugma";
            spritesheet_frames = 6;
            speed = 1.0;
            isfake = false
        }

        PokemonType::Magcargo => {
            spritesheet_name = "magcargo";
            spritesheet_frames = 4;
            speed = 2.0;
            isfake = false
        }
        // PokemonType::Diglett =>
        // {
        //     speed = 0;
        //     // spr_num_frames = 5;
        //     isfake = false
        // },

        // PokemonType::Dugtrio =>
        // {
        //     speed = 0;
        //     // spr_num_frames = 5;
        //     isfake = false
        // },

        // PokemonType::Voltorb =>
        // {
        //     speed = 2;
        //     // spr_num_frames = 4;
        //     isfake = false
        // },

        // PokemonType::Electrode =>
        // {
        //     speed = 2;
        //     // spr_num_frames = 4;
        //     isfake = false
        // },

        // PokemonType::Pichu =>
        // {
        //     speed = 2;
        //     // spr_num_frames = 6;
        //     isfake = false
        // },

        // PokemonType::Pikachu =>
        // {
        //     speed = 3;
        //     // spr_num_frames = 6;
        //     isfake = false
        // },

        // PokemonType::Togepi =>
        // {
        //     speed = 1;
        //     // spr_num_frames = 6;
        //     isfake = false
        // },

        // PokemonType::Doduo =>
        // {
        //     speed = 4;
        //     // spr_num_frames = 6;
        //     isfake = false
        // },
    }

    sprite.load_sprites(ctx, &spritesheet_name, spritesheet_frames);
    sprite.sprite_count = spritesheet_frames;

    Pokemon {
        pokemon_type,
        sprite,
        position: Point2::new(0.0, 0.0),
        speed,
        isfake: false,
    }
}

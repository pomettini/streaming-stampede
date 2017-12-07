use ggez::Context;
use ggez::graphics::{Point};

use globals::*;

impl Pokemon
{
    pub fn new(ctx: &mut Context) -> Pokemon 
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
        if self.spr_frame as u8 >= self.spr_num_frames
        {
            self.spr_frame = 0.;
        }

        // if(self.pos.x < 0.)
        // {
        //     self.pos.x = WINDOW_H as f32;
        // }
    }
}
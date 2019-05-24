use event::*;
use ggez::*;

pub struct MenuScene {}

impl MenuScene {
    pub fn new(_ctx: &mut Context) -> GameResult<MenuScene> {
        Ok(MenuScene {})
    }
}

impl EventHandler for MenuScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        println!("Ciao");
        Ok(())
    }
}

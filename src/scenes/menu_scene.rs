use ggez::*;
use event::*;

pub struct MenuScene
{

}

impl MenuScene
{
    pub fn new(ctx: &mut Context) -> GameResult<MenuScene>
    {
        Ok(MenuScene
        {

        })
    }
}

impl EventHandler for MenuScene
{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        println!("Ciao");
        Ok(())
    }
}
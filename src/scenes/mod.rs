use event::*;
use ggez::*;

pub mod menu_scene;

use menu_scene::*;

// Took inspiration from Rustris https://github.com/obsoke/rustris/blob/master/src/states/mod.rs

pub struct SceneManager {
    running: bool,
    scenes: Vec<Box<EventHandler>>,
}

impl SceneManager {
    pub fn new(ctx: &mut Context) -> SceneManager {
        let scene = Box::new(MenuScene::new(ctx).unwrap());

        SceneManager {
            running: true,
            scenes: vec![scene],
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn quit(&mut self) {
        self.scenes.clear();
        self.running = false
    }
}

impl EventHandler for SceneManager {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for (_, scene) in self.scenes.iter_mut().enumerate() {
            scene.draw(ctx)?;
        }

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(scene) = self.scenes.last_mut() {
            scene.key_down_event(ctx, keycode, _keymod, _repeat);
        }
    }
}

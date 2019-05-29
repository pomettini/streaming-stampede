use ggez::*;
use ggez::graphics::*;

pub struct Assets {
    pub font: Font,
    pub background: Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Assets {
        let font = Assets::init_font(ctx);
        let background = Assets::init_background(ctx);

        Assets {
            font,
            background,
        }
    }

    pub fn init_font(ctx: &mut Context) -> Font {
        Font::new(ctx, "/Pacifico.ttf", 48).unwrap()
    }

    pub fn init_background(ctx: &mut Context) -> Image {
        Image::new(ctx, "/field_bg.png").unwrap()
    }

    /* pub fn init_sprites(ctx: &mut Context) -> Image {

    } */
}

use ggez::*;

pub struct Assets {
    pub font: graphics::Font,
    pub background: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Assets {
        let font = init_font(ctx);
        let background = init_background(ctx);

        Assets {
            font: font,
            background: background,
        }
    }
}

pub fn init_font(ctx: &mut Context) -> graphics::Font {
    graphics::Font::new(ctx, "/Pacifico.ttf", 48).unwrap()
}

pub fn init_background(ctx: &mut Context) -> graphics::Image {
    graphics::Image::new(ctx, "/field_bg.png").unwrap()
}

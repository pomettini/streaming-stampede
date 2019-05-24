use ggez::*;

use ggez::graphics::*;

use assets::*;
use player::*;
use utils::*;

pub fn draw_players_counter(ctx: &mut Context, players: &Vec<Player>, assets: &Assets) {
    for player in players {
        draw_player_counter(ctx, player, assets);
    }
}

fn draw_player_counter(ctx: &mut Context, player: &Player, assets: &Assets) -> GameResult<()> {
    // Slow casting, needs refactor
    let player_counter = player.get_counter();
    let player_counter_position = world_to_screen(player.get_score_position());
    let text = graphics::Text::new(ctx, player_counter.to_string().as_str(), &assets.font)?;
    graphics::draw(ctx, &text, player_counter_position, 0.0)
}

pub fn draw_background(ctx: &mut Context, assets: &Assets) -> GameResult<()> {
    graphics::draw(ctx, &assets.background, Point2::origin(), 0.0)
}

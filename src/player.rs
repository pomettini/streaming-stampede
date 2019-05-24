use ggez::event::*;
use ggez::*;

use ggez::graphics::Point2;

pub struct Player {
    score: usize,
    counter: usize,
    keycode: event::Keycode,
    score_position: Point2,
}

impl Player {
    pub fn new() -> Player {
        Player {
            score: 0,
            counter: 0,
            keycode: event::Keycode::Escape,
            // This needs to be put elsewhere
            score_position: Point2::new(0.0, 0.0),
        }
    }

    pub fn get_score(&self) -> &usize {
        &self.score
    }

    pub fn get_counter(&self) -> &usize {
        &self.counter
    }

    pub fn get_key(&self) -> &event::Keycode {
        &self.keycode
    }

    pub fn set_key(&mut self, key: Keycode) {
        self.keycode = key;
    }

    pub fn increase_score(&mut self, amount: usize) {
        self.score += amount;
    }

    pub fn increase_counter(&mut self) {
        self.counter += 1;
    }

    pub fn reset_score(&mut self) {
        self.score = 0;
    }

    pub fn reset_counter(&mut self) {
        self.counter = 1;
    }

    pub fn get_score_position(&self) -> &Point2 {
        &self.score_position
    }

    pub fn set_score_position(&mut self, point: Point2) {
        self.score_position = point;
    }
}

pub fn create_players() -> Vec<Player> {
    // Maybe I should refactor this one
    // MY GOSH

    let mut players: Vec<Player> = Vec::new();

    let mut first_player = Player::new();
    let mut second_player = Player::new();
    let mut third_player = Player::new();
    let mut fourth_player = Player::new();

    first_player.set_score_position(Point2::new(0.2, 0.2));
    second_player.set_score_position(Point2::new(0.4, 0.2));
    third_player.set_score_position(Point2::new(0.6, 0.2));
    fourth_player.set_score_position(Point2::new(0.8, 0.2));

    first_player.set_key(Keycode::A);
    second_player.set_key(Keycode::S);
    third_player.set_key(Keycode::D);
    fourth_player.set_key(Keycode::F);

    players.push(first_player);
    players.push(second_player);
    players.push(third_player);
    players.push(fourth_player);

    players
}

use constants::*;
use ggez::graphics::*;

pub fn world_to_screen(world: Point2) -> Point2 {
    Point2::new(
        (WINDOW_WIDTH as f32) * world.x,
        (WINDOW_HEIGHT as f32) * world.y,
    )
}

pub fn screen_to_world(screen: Point2) -> Point2 {
    Point2::new(
        screen.x / (WINDOW_WIDTH as f32),
        screen.y / (WINDOW_HEIGHT as f32),
    )
}

#[test]
fn test_world_to_screen() {
    let world = Point2::new(0.5, 0.5);
    let screen = Point2::new(512.0, 384.0);
    assert_eq!(world_to_screen(world), screen);
}

#[test]
fn test_screen_to_world() {
    let screen = Point2::new(512.0, 384.0);
    let world = Point2::new(0.5, 0.5);
    assert_eq!(screen_to_world(screen), world);
}

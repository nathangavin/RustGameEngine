use specs::prelude::*;
use specs::derive::Component;
use sdl2::rect::{Point, Rect};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Position(Point);
impl Component for Position {
    type Storage = VecStorage<self>;
}

struct Velocity {
    speed: i32,
    direction: Direction,
}
impl Component for Velocity {
    type Storage = VecStorage<self>;
}

struct Sprite {
    spritesheet: usize,
    region: Rect,
}
impl Component for Sprite {
    type Storage = VecStorage<self>;
}

struct MovementAnimation {
    current_frame: usize,
    up_frames: Vec<Sprite>,
    down_frames: Vec<Sprite>,
    left_frames: Vec<Sprite>,
    right_frames: Vec<Sprite>,
}
impl Component for MovementAnimation {
    type Storage = VecStorage<self>;
}
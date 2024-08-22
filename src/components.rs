use specs::prelude::*;
use sdl2::rect::{FPoint, Point, Rect};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Debug)]
pub struct KeyboardControlled;
impl Component for KeyboardControlled {
    type Storage = NullStorage<Self>;
}

/*
pub struct Enemy;
impl Component for Enemy {
    type Storage = NullStorage<Self>;
}

pub struct Position(pub Point);
impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
*/

pub struct Scale(pub i32);
impl Component for Scale {
    type Storage = VecStorage<Self>;
}

pub struct Mass(pub f32);
impl Component for Mass {
    type Storage = VecStorage<Self>;
}

pub struct CelestialBody {
    pub radius: f32,
}
impl Component for CelestialBody {
    type Storage = VecStorage<Self>;
}

pub struct Position(pub FPoint);
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub x_speed: f32,
    pub y_speed: f32
}
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

pub struct Forces(pub Vec<(f32, f32)>);
impl Component for Forces {
    type Storage = VecStorage<Self>;
}


#[derive(Debug)]
pub struct Acceleration {
    pub x_accel: f32,
    pub y_accel: f32
}
impl Component for Acceleration {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy)]
pub struct OrbitalPath {
    pub centre: (f32, f32),
    pub radius: f32,
    pub angle: f32,
    pub rotation_speed: f32 
}
pub struct OrbitalPaths(pub Vec<OrbitalPath>);
impl Component for OrbitalPaths {
    type Storage = VecStorage<Self>;
}

pub struct Polygon(pub Vec<FPoint>);
impl Component for Polygon {
    type Storage = VecStorage<Self>;
}

/*
#[derive(Clone, Copy)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
}
impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

pub struct MovementAnimation {
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}
impl Component for MovementAnimation {
    type Storage = VecStorage<Self>;
}
*/

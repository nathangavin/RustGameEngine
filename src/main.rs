use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;

use specs::prelude::*;
use specs::storage::VecStorage;
use specs_derive::Component;

const PLAYER_MOVEMENT_SPEED: i32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
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

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
    current_frame: i32,
}

fn direction_spreadsheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2
    }
}

fn render(
    canvas: &mut WindowCanvas, 
    color: Color, 
    texture: &Texture,
    player: &Player) -> Result<(), String> {

        canvas.set_draw_color(color);
        canvas.clear();

        let (width, height) = canvas.output_size()?;

        let (frame_width, frame_height) = player.sprite.size();
        let current_frame = Rect::new(
            player.sprite.x() + frame_width as i32 * player.current_frame,
            player.sprite.y() + frame_height as i32 * direction_spreadsheet_row(player.direction),
            frame_width,
            frame_height
        );
        
        let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
        canvas.copy(texture, current_frame, screen_rect)?;

        canvas.present();
        Ok(())
}


fn update_player(player: &mut Player) {
    match player.direction {
        Direction::Down => {
            player.position = player.position.offset(0, player.speed);
        },
        Direction::Up => {
            player.position = player.position.offset(0, -player.speed);
        },
        Direction::Left => {
            player.position = player.position.offset(-player.speed, 0);
        },
        Direction::Right => {
            player.position = player.position.offset(player.speed, 0);
        },
    }

    if player.speed != 0 {
        player.current_frame  = (player.current_frame + 1) % 3;
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialise video subsystem");


    let mut canvas =  window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;
    
    let mut player = Player {
        position: Point::new(0,0),
        sprite: Rect::new(0,0, 26, 36),
        speed: 0,
        direction: Direction::Right,
        current_frame: 0,
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop {
        // handling events
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Left;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Right;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Up;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Down;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, ..} => {
                    player.speed = 0;
                },
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;
        update_player(&mut player);

        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;


        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));

    }

    Ok(())
}

mod components;
mod physics;
mod animator;
mod keyboard;
mod renderer;
mod ai;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::FPoint;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use specs::prelude::*;

use crate::components::*;

pub enum ScaleCommand {
    Reduce,
    Increase
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

fn initialise_planet(world: &mut World, 
                        position: FPoint, 
                        mass: f32, 
                        radius: f32,
                        orbital_paths: Vec<OrbitalPath>) {

    if orbital_paths.is_empty() {
        world.create_entity()
        .with(Mass(mass))
        .with(CelestialBody { radius })
        .with(Position(position))
        .build();

    } else {
        world.create_entity()
        .with(Mass(mass))
        .with(CelestialBody { radius })
        .with(OrbitalPaths(orbital_paths))
        .build();
    }
} 

fn initialise_free_body(world: &mut World,
                            position: FPoint,
                            mass: f32,
                            vertices: &Vec<FPoint>,
                            velocity: (f32, f32)) {

    world.create_entity()
        .with(Position(position))
        .with(Mass(mass))
        .with(Polygon(vertices.to_vec()))
        .with(Velocity {x_speed: velocity.0, y_speed: velocity.1})
        .with(Acceleration { x_accel: 0.0, y_accel: 0.0})
        .with(Forces(Vec::new()))
        .build();
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

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        //.with(ai::AI, "AI", &[])
        //.with(physics::Physics, "Physics", &["Keyboard", "AI"])
        .with(physics::Physics, "Physics", &[])
        .with(animator::Animator, "Animator", &["Physics"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    let scale_command : Option<ScaleCommand> = None;
    world.insert(scale_command);

    world.create_entity()
        .with(Scale(0))
        .build();

    /*
    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
        texture_creator.load_texture("assets/reaper.png")?,
    ];
    */

    let rail = OrbitalPath {
        centre: (0.0, 0.0),
        radius: 1000.0,
        angle: 0.0,
        rotation_speed: 0.002
    };
    let rail2 = OrbitalPath {
        centre: (0.0, 0.0),
        radius: 1200.0,
        angle: 0.0,
        rotation_speed: -0.001
    };
    let inner_rail = OrbitalPath {
        centre: (0.0,0.0),
        radius: 50.0,
        angle: 0.0,
        rotation_speed: 0.01
    };
    let cos_30 = (1_f32 / 12_f32).cos();
    let sin_30 = (1_f32 / 12_f32).sin();
    initialise_planet(&mut world, FPoint::new(0.0,-200.0), 6e12, 50.0, vec![]);
    initialise_planet(&mut world, FPoint::new(-200.0 * cos_30,200.0 * sin_30), 6e12, 50.0, vec![]);
    initialise_planet(&mut world, FPoint::new(200.0 * cos_30,200.0 * sin_30), 6e12, 50.0, vec![]);
    initialise_planet(&mut world, FPoint::new(0.0,0.0), 6e10, 20.0, vec![rail.clone()]);
    initialise_planet(&mut world, FPoint::new(0.0,0.0), 2e10, 10.0, vec![rail2]);
    initialise_planet(&mut world, FPoint::new(0.0,0.0), 6e8, 5.0, vec![rail.clone(), inner_rail]);

    let vertices = vec![
        FPoint::new(-10.0, -10.0),
        FPoint::new(-10.0, 10.0),
        FPoint::new(10.0, 10.0),
        FPoint::new(10.0, -10.0),
    ];
   initialise_free_body(&mut world, FPoint::new(1000.0,0.0), 1e5, &vertices, (0.0, 1.0));
   initialise_free_body(&mut world, FPoint::new(1100.0,100.0), 1e5, &vertices, (-1.0, 1.0));

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    //let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Uh OH");
    let mut timestamp = SystemTime::now();

    'running: loop {
        // handling events

        let mut scale_command: Option<ScaleCommand> = None;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::UP), ..} => {
                    scale_command = Some(ScaleCommand::Increase);
                },
                Event::KeyDown { keycode: Some(Keycode::DOWN), ..} => {
                    scale_command = Some(ScaleCommand::Reduce);
                },
                Event::KeyUp { keycode: Some(Keycode::UP), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::DOWN), repeat: false, ..} => {
                    scale_command = None;
                },
                _ => {}
            }
        }

        *world.write_resource() = scale_command;


        // Update
        i = (i + 1) % 255;
        dispatcher.dispatch(&mut world);
        world.maintain();

        let current_time = SystemTime::now().duration_since(timestamp).expect("time went backwards");
        //println!("{}", 1000 / current_time.as_millis());
        timestamp = SystemTime::now();

        

        // Render
       renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), world.system_data())?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }

    Ok(())
}

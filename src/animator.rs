use std::{f32::consts::PI};

use specs::prelude::*;

use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        ReadStorage<'a, Mass>,
        ReadStorage<'a, CelestialBody>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, OrbitalPath>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Polygon>);

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;

        // update position of bodies on rails
        for rail in (&mut data.3).join() {
            let angle = rail.angle + (2.0 * PI * rail.rotation_speed);
            if angle > (2.0 * PI) {
                rail.angle = 0.0;
            } else {
                rail.angle = angle;
            }
        } 

        // update position of free bodies
        for (position, velocity) in (&mut data.2, &data.4).join() {
            position.0.x += velocity.x_speed;
            position.0.y += velocity.y_speed;
            println!("{:?}", velocity);
        }

        /*
        for (anim, sprite, vel) in (&mut data.0, &mut data.1, &data.2).join() {
            if vel.speed == 0 {
                continue;
            }


            let frames = match vel.direction {
                Left => &anim.left_frames,
                Right => &anim.right_frames,
                Up => &anim.up_frames,
                Down => &anim.down_frames
            };

            anim.current_frame = (anim.current_frame + 1) % frames.len();
            *sprite = frames[anim.current_frame].clone();
        }
        */
    }
}

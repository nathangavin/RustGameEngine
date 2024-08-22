use std::{f32::consts::PI};

use specs::prelude::*;

use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        ReadStorage<'a, Mass>,
        ReadStorage<'a, CelestialBody>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, OrbitalPaths>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Polygon>,
        );

    fn run(&mut self, mut data: Self::SystemData) {

        // update position of bodies on rails
        for rails in (&mut data.3).join() {
            for rail in rails.0.as_mut_slice() {
                let angle = rail.angle + (2.0 * PI * rail.rotation_speed);
                if angle > (2.0 * PI) {
                    rail.angle = 0.0;
                } else if angle < 0.0 {
                    rail.angle = 2.0 * PI;
                } else {
                    rail.angle = angle;
                }
            }
        } 

        // update position of free bodies
        for (position, velocity) in (&mut data.2, &data.4).join() {
            position.0.x += velocity.x_speed;
            position.0.y += velocity.y_speed;
        }
    }
}

use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (
    ReadStorage<'a, Mass>,
    ReadStorage<'a, CelestialBody>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, OrbitalPath>,
    WriteStorage<'a, Velocity>,
    ReadStorage<'a, Polygon>);

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;

        /*
        for (pos, vel) in (&mut data.0, &data.1).join() {
            match vel.direction {
                Left => {
                    pos.0 = pos.0.offset(-vel.speed, 0);
                },
                Right => {
                    pos.0 = pos.0.offset(vel.speed, 0);
                },
                Up => {
                    pos.0 = pos.0.offset(0, -vel.speed);
                },
                Down => {
                    pos.0 = pos.0.offset(0, vel.speed);
                },
            }
        }
        */
        
        let grav_constant : f32 = 6.67e-11;

        for (mass, position, velocity) in (&data.0, &data.2, &mut data.4).join() {
            let mut summed_gravity_force = (0.0,0.0);
            // sum for fixed bodies
            for (second_mass, second_position, cbody) in (&data.0, &data.2, &data.1).join() {
                let diff_x = second_position.0.x - position.0.x; 
                let diff_y = second_position.0.y - position.0.y;
                
                let diff_h = (diff_x * diff_x + diff_y * diff_y).sqrt();
                
                let gravity_force = (grav_constant * mass.0 * second_mass.0) / (diff_h * diff_h);
                let g_force_x = gravity_force * (diff_x / diff_h);
                let g_force_y = gravity_force * (diff_y / diff_h);
                summed_gravity_force.0 = g_force_x;
                summed_gravity_force.1 = g_force_y;
            }   
            // sum for bodies on orbital rail paths
            for (second_mass,  rail) in (&data.0, &data.3).join() {
                let second_x = rail.centre.0 + rail.radius * rail.angle.cos();
                let second_y = rail.centre.1 + rail.radius * rail.angle.sin();
                
                let diff_x = second_x - position.0.x;
                let diff_y = second_y - position.0.x;

                let diff_h = (diff_x * diff_x + diff_y * diff_y).sqrt();

                let gravity_force = (grav_constant * mass.0 * second_mass.0) / (diff_h * diff_h);
                let g_force_x = gravity_force * (diff_x / diff_h);
                let g_force_y = gravity_force * (diff_y / diff_h);
                summed_gravity_force.0 = g_force_x;
                summed_gravity_force.1 = g_force_y;
            }

            let accel_x = summed_gravity_force.0 / mass.0;
            println!("{}", accel_x);
            let accel_y = summed_gravity_force.1 / mass.0;
            println!("{}", accel_y);

            velocity.x_speed += accel_x;
            velocity.y_speed += accel_y;
        }
    }
}

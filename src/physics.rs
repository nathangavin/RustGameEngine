use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (
    ReadStorage<'a, Mass>,
    ReadStorage<'a, CelestialBody>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, OrbitalPaths>,
    WriteStorage<'a, Velocity>,
    ReadStorage<'a, Polygon>,
    WriteStorage<'a, Acceleration>,
    WriteStorage<'a, Forces>);

    fn run(&mut self, mut data: Self::SystemData) {
        
        let grav_constant : f32 = 6.67e-11;

        for (mass, position, velocity, acceleration, forces) in (&data.0, &data.2, &mut data.4, &mut data.6, &mut data.7).join() {
            forces.0.clear();
            let mut summed_gravity_force = (0.0,0.0);
            // sum for fixed bodies
            for (second_mass, second_position, cbody) in (&data.0, &data.2, &data.1).join() {
                let diff_x = second_position.0.x - position.0.x; 
                let diff_y = second_position.0.y - position.0.y;
                
                let diff_h = (diff_x * diff_x + diff_y * diff_y).sqrt();
                
                let gravity_force = (grav_constant * mass.0 * second_mass.0) / (diff_h * diff_h);
                let g_force_x = gravity_force * (diff_x / diff_h);
                let g_force_y = gravity_force * (diff_y / diff_h);
                forces.0.push((g_force_x, g_force_y));
                summed_gravity_force.0 += g_force_x;
                summed_gravity_force.1 += g_force_y;
            }   

            // sum for bodies on orbital rail paths
            for (third_mass,  rails) in (&data.0, &data.3).join() {
                let mut third_x = 0.0;
                let mut third_y = 0.0;
                for rail in rails.0.as_slice() {
                    third_x += rail.centre.0 + rail.radius * rail.angle.cos();
                    third_y += rail.centre.1 + rail.radius * rail.angle.sin();

                }
                
                let diff_x = third_x - position.0.x;
                let diff_y = third_y - position.0.y;

                let diff_h = (diff_x * diff_x + diff_y * diff_y).sqrt();

                let gravity_force = (grav_constant * mass.0 * third_mass.0) / (diff_h * diff_h);
                let g_force_x = gravity_force * (diff_x / diff_h);
                let g_force_y = gravity_force * (diff_y / diff_h);

                forces.0.push((g_force_x, g_force_y));
                summed_gravity_force.0 += g_force_x;
                summed_gravity_force.1 += g_force_y;
            }
            
            let accel_x = summed_gravity_force.0 / mass.0;
            let accel_y = summed_gravity_force.1 / mass.0;

            acceleration.x_accel = accel_x;
            acceleration.y_accel = accel_y;

            velocity.x_speed += accel_x;
            velocity.y_speed += accel_y;
        }
    }
}

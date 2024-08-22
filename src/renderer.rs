use std::f32::consts::PI;

use sdl2::libc::printf;
use specs::prelude::*;
use sdl2::rect::{FPoint, Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas,Texture};

use crate::components::*;

pub type SystemData<'a> = (
    ReadStorage<'a, Mass>,
    ReadStorage<'a, CelestialBody>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, OrbitalPaths>,
    ReadStorage<'a, Velocity>,
    ReadStorage<'a, Polygon>,
    ReadStorage<'a, Acceleration>,
    ReadStorage<'a, Forces>,
    ReadStorage<'a, Scale>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    data: SystemData) -> Result<(), String> {

        canvas.set_draw_color(background);
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let (width, height) = canvas.output_size()?;

        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;

        let mut calculated_scale = 1.0; 
        for scale in (&data.8).join() {
            calculated_scale = 2_f32.powi(scale.0);
        }

        // draw fixed bodies
        for (cbody, fixed_pos) in (&data.1, &data.2).join() {
            let scaled_x = fixed_pos.0.x * calculated_scale;
            let scaled_y = fixed_pos.0.y * calculated_scale;
            let screen_position = FPoint::new(scaled_x + half_width, 
                                    scaled_y + half_height);
            let scaled_radius = cbody.radius * calculated_scale;
            draw_circle(canvas, screen_position, scaled_radius, 100).unwrap();
        }

        // draw bodies on rails
        for (cbody, rails) in (&data.1, &data.3).join() {
            let mut body_position = FPoint::new(half_width, half_height);
            for rail in rails.0.as_slice() {
                body_position.x += (rail.centre.0 + (rail.radius * rail.angle.cos())) * calculated_scale;
                body_position.y += (rail.centre.1 + (rail.radius * rail.angle.sin())) * calculated_scale;
            }
            let scaled_radius = cbody.radius * calculated_scale;
            draw_circle(canvas, body_position, scaled_radius, 100).unwrap();
        }

        // draw free bodies
        for (pos, polygon, acceleration, forces) in (&data.2, &data.5, &data.6, &data.7).join() {
            let mut f_points = vec![FPoint::new(0.0,0.0); polygon.0.len() + 1];
            for (i,point) in polygon.0.iter().enumerate() {
                f_points[i].x = half_width + (pos.0.x + point.x) * calculated_scale;
                f_points[i].y = half_height + (pos.0.y + point.y) * calculated_scale;
            } 
            
            f_points[polygon.0.len()].x = half_width + (pos.0.x + polygon.0[0].x) * calculated_scale;
            f_points[polygon.0.len()].y = half_height + (pos.0.y + polygon.0[0].y) * calculated_scale;

            canvas.draw_flines(f_points.as_slice()).unwrap();
            
            // draw forces as vectors multiplied by 10 
            for force in forces.0.as_slice() {
                canvas.draw_fline(
                            FPoint::new(
                                half_width + pos.0.x * calculated_scale, 
                                half_height + pos.0.y * calculated_scale),
                            FPoint::new(
                                half_width + (pos.0.x + (10.0 * force.0)) * calculated_scale,
                                half_height + (pos.0.y + (10.0 * force.1)) * calculated_scale)).unwrap();
            }
        }

        canvas.present();

        Ok(())
}



fn draw_circle(canvas: &mut WindowCanvas, position: FPoint, radius: f32, steps: usize) -> Result<(), String> {
    let mut f_points = vec![FPoint::new(0.0, 0.0); steps + 1];
   
    let two_pi = 2.0 * PI;
    for pos in 0..steps {
        let angle = (pos as f32 * two_pi) / steps as f32;
        f_points[pos].x = position.x + radius * angle.cos();
        f_points[pos].y = position.y + radius * angle.sin();
        
    }

    f_points[steps].x = position.x + radius;
    f_points[steps].y = position.y;

    canvas.draw_flines(f_points.as_slice())
}

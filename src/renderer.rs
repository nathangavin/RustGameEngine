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
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    data: SystemData) -> Result<(), String> {

        canvas.set_draw_color(background);
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let (width, height) = canvas.output_size()?;

        /*
        for (pos, sprite) in (&data.0, &data.1).join() {
            let current_frame = sprite.region;

            let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
            let screen_rect = Rect::from_center(screen_position, 
                                                        current_frame.width(), 
                                                        current_frame.height());
            canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
        }
        */

        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;

        // draw fixed bodies
        for (cbody, fixed_pos) in (&data.1, &data.2).join() {
            let screen_position = FPoint::new(fixed_pos.0.x + half_width, 
                                    fixed_pos.0.y + half_height);
            draw_circle(canvas, screen_position, cbody.radius, 100).unwrap();
        }

        // draw bodies on rails
        for (cbody, rails) in (&data.1, &data.3).join() {
            let mut body_position = FPoint::new(half_width, half_height);
            for rail in rails.0.as_slice() {
                body_position.x += rail.centre.0 + (rail.radius * rail.angle.cos());
                body_position.y += rail.centre.1 + (rail.radius * rail.angle.sin());
            }

            draw_circle(canvas, body_position, cbody.radius, 100).unwrap();
        }

        // draw free bodies
        for (pos, polygon, acceleration, forces) in (&data.2, &data.5, &data.6, &data.7).join() {
            let mut f_points = vec![FPoint::new(0.0,0.0); polygon.0.len() + 1];
            for (i,point) in polygon.0.iter().enumerate() {
                f_points[i].x = half_width + pos.0.x + point.x;
                f_points[i].y = half_height + pos.0.y + point.y;
            } 
            
            f_points[polygon.0.len()].x = half_width + pos.0.x + polygon.0[0].x;
            f_points[polygon.0.len()].y = half_height + pos.0.y + polygon.0[0].y;

            canvas.draw_flines(f_points.as_slice()).unwrap();
            
            // draw forces as vectors multiplied by 10 
            for force in forces.0.as_slice() {
                canvas.draw_fline(
                            FPoint::new(
                                half_width + pos.0.x, 
                                half_height + pos.0.y),
                            FPoint::new(
                                half_width + pos.0.x + (10.0 * force.0),
                                half_height + pos.0.y + (10.0 * force.1))).unwrap();

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

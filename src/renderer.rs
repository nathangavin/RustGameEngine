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
    ReadStorage<'a, OrbitalPath>,
    ReadStorage<'a, Velocity>,
    ReadStorage<'a, Polygon>,

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

        for (pos, polygon) in (&data.0, &data.2).join() {
            let centre = Point::new(width as i32 / 2, height as i32 /2);
            for (i, vertex) in polygon.0.iter().enumerate() {
                match polygon.0.get(i+1) {
                    Some(n_v) => canvas.draw_line(*vertex + centre, *n_v + centre)?,
                    None => ()
                }
            }
        
            match polygon.0.last() {
                Some(last) => {
                    match polygon.0.first() {
                        Some(first) => {
                            canvas.draw_line(*last + centre, *first + centre)?;
                        },
                        None => ()
                    }
                },
                None => ()
            }
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
        for (cbody, rail) in (&data.1, &data.3).join() {
            let body_position = FPoint::new(half_width + rail.centre.0 + (rail.radius * rail.angle.cos()),
                half_height + rail.centre.1 + (rail.radius * rail.angle.sin()));  

            draw_circle(canvas, body_position, cbody.radius, 100).unwrap();
        }

        // draw free bodies
        for (pos, velocity, polygon) in (&data.2, &data.4, &data.5).join() {
            let mut f_points = vec![FPoint::new(0.0,0.0); polygon.0.len() + 1];
            for (i,point) in polygon.0.iter().enumerate() {
                f_points[i].x = half_width + pos.0.x + point.x;
                f_points[i].y = half_height + pos.0.y + point.y;
            } 
            
            f_points[polygon.0.len()].x = half_width + pos.0.x + polygon.0[0].x;
            f_points[polygon.0.len()].y = half_height + pos.0.y + polygon.0[0].y;

            canvas.draw_flines(f_points.as_slice()).unwrap();
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

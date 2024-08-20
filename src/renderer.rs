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
    ReadStorage<'a, FixedPosition>,
    ReadStorage<'a, OrbitalRailPosition>,

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
        for (cbody, fixed_pos) in (&data.1, &data.2).join() {
            let screen_position = (fixed_pos.x + half_width, 
                                    fixed_pos.y + half_height);
            draw_circle(canvas, screen_position, cbody.radius, 100).unwrap();
        }

        for (cbody, rail) in (&data.1, &data.3).join() {
            let body_position = (half_width + rail.centre.0 + (rail.radius * rail.angle.cos()),
                half_height + rail.centre.1 + (rail.radius * rail.angle.sin()));  

            draw_circle(canvas, body_position, cbody.radius, 100).unwrap();
        }

        canvas.present();

        Ok(())
}


fn draw_circle(canvas: &mut WindowCanvas, position: (f32, f32), radius: f32, steps: usize) -> Result<(), String> {
    let mut f_points = vec![FPoint::new(0.0, 0.0); steps + 1];
   
    let two_pi = 2.0 * PI;
    for pos in 0..steps {
        let angle = (pos as f32 * two_pi) / steps as f32;
        f_points[pos].x = position.0 + radius * angle.cos();
        f_points[pos].y = position.1 + radius * angle.sin();
    }

    f_points[steps].x = position.0 + radius;
    f_points[steps].y = position.1;

    canvas.draw_flines(f_points.as_slice())
}

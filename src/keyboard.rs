use specs::prelude::*;

use crate::{components::*, ScaleCommand};

const SCALE_STEP : i32 = 1;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<ScaleCommand>>,
        WriteStorage<'a, Scale>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let scale_command = match &*data.0 {
            Some(scale_command) => scale_command,
            None => return,
        };

        for scale in (&mut data.1).join() {
            match scale_command {
                ScaleCommand::Reduce => {scale.0 = scale.0 - SCALE_STEP},
                ScaleCommand::Increase => {scale.0 = scale.0 + SCALE_STEP}
            }
        }
    }
}

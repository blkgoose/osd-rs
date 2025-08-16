use crate::config::Kind::{self, Brightness, Volume};

pub enum Command {
    BrightnessUp(i32),
    BrightnessDown(i32),
    VolumeUp(i32),
    VolumeDown(i32),
}

impl Command {
    pub(crate) fn up(kind: &Kind, val: i32) -> Self {
        match kind {
            Brightness => Command::BrightnessUp(val),
            Volume => Command::VolumeUp(val),
        }
    }

    pub(crate) fn down(kind: &Kind, val: i32) -> Self {
        match kind {
            Brightness => Command::BrightnessDown(val),
            Volume => Command::VolumeDown(val),
        }
    }
}

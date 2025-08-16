use std::sync::mpsc::Sender;
use std::thread;

use crate::{
    command::Command,
    config::{DisplayMethod, FileConfig},
    utils::common_watcher,
};

pub fn watch(config: FileConfig, tx: Sender<(Command, DisplayMethod)>) {
    thread::spawn(move || {
        common_watcher(
            || {
                let path: &str = &config.path;
                let max = config.max as f32;
                let content = std::fs::read_to_string(path)
                    .unwrap_or_default()
                    .trim()
                    .to_owned();

                let parsed_value = content.parse::<i32>().unwrap_or_default() as f32;

                (parsed_value / max * 100.0) as i32
            },
            &config.kind,
            tx,
            config.interval,
            config.display_with,
        )
    });
}

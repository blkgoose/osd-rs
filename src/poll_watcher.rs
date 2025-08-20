use std::sync::mpsc::Sender;
use std::thread;

use crate::{
    command::Command,
    config::{DisplayMethod, PollConfig},
    utils::common_watcher,
};

pub fn watch(config: PollConfig, tx: Sender<(Command, DisplayMethod)>) {
    thread::spawn(move || {
        common_watcher(
            || {
                let command: &str = &config.command;

                let max = config.common.max as f32;
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()
                    .expect(&format!("Failed to execute command: {}", command));

                let stdout = String::from_utf8(output.stdout).unwrap_or_default();

                let parsed_value = stdout.parse::<i32>().unwrap_or_default() as f32;

                (parsed_value / max * 100.0) as i32
            },
            &config.common,
            tx,
        )
    });
}

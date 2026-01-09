use std::thread;
use std::{io::BufRead, sync::mpsc::Sender};

use crate::{
    command::Command,
    config::{DisplayMethod, PipeConfig},
};

pub fn watch(config: PipeConfig, tx: Sender<(Command, DisplayMethod)>) {
    thread::spawn(move || {
        let max = config.common.max as f32;

        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(&config.command)
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start pipe command");

        let stdout = child
            .stdout
            .take()
            .expect("Failed to capture stdout of pipe command");

        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(content) = line {
                let parsed_value = content.trim().parse::<f32>().unwrap_or_default();
                let value = (parsed_value / max * 100.0) as i32;

                if config.common.debug {
                    println!("Pipe output: {}, Parsed value: {}", content.trim(), value);
                }

                let command = Command::new(config.common.tag.clone(), value);
                tx.send((command, config.common.display_with)).ok();
            }
        }
    });
}

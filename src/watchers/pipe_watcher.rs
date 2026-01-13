use std::io::BufRead;
use std::thread;

use crate::{
    command::{Command, CommandSender},
    config::PipeConfig,
};

pub fn watch(config: PipeConfig, tx: CommandSender) {
    thread::spawn(move || {
        let max = config.common.max as f32;

        let mut prev: Option<i32> = None;

        let stdout = std::process::Command::new("sh")
            .arg("-c")
            .arg(&config.command)
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start pipe command")
            .stdout
            .take()
            .expect("Failed to capture stdout of pipe command");

        let reader = std::io::BufReader::new(stdout);
        for content in reader.lines().map_while(Result::ok) {
            let parsed_value = content.trim().parse::<f32>().unwrap_or_default();
            let value = (parsed_value / max * 100.0) as i32;

            if config.common.debug {
                println!("Pipe output: {}, Parsed value: {}", content.trim(), value);
            }

            match prev {
                None => {}
                Some(prev_value) if prev_value == value => {}
                Some(prev) => {
                    if config.common.debug {
                        println!("Value changed from {:?} to {}", prev, value);
                    }
                    let command = Command::new(config.common.tag.clone(), value);
                    tx.send((command, config.common.display_with)).ok();
                }
            }
            prev = Some(value);
        }
    });
}

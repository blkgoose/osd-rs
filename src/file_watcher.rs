use std::sync::mpsc::Sender;
use std::thread;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use crate::{
    command::Command,
    config::{DisplayMethod, FileConfig},
};

pub fn watch(config: FileConfig, tx: Sender<(Command, DisplayMethod)>) {
    thread::spawn(move || {
        let max = config.common.max as f32;

        let prev = get_file_value(&config.path, max, config.common.debug);

        let (watcher_tx, watcher_rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(watcher_tx, Config::default())
            .expect("Failed to create watcher");

        watcher
            .watch(config.path.as_ref(), RecursiveMode::Recursive)
            .expect("Failed to watch file");

        for res in watcher_rx {
            match res {
                Ok(event) => {
                    if event.kind.is_modify() {
                        let curr = get_file_value(&config.path, max, config.common.debug);

                        if curr != prev {
                            let command = Command::new(config.common.tag.clone(), curr);
                            tx.send((command, config.common.display_with)).ok();
                        }
                    }
                }
                Err(error) => println!("Error: {error:?}"),
            }
        }
    });
}

fn get_file_value(path: &str, max: f32, debug: bool) -> i32 {
    let content = std::fs::read_to_string(path)
        .unwrap_or_default()
        .trim()
        .to_owned();

    if debug {
        println!("Reading file: {}, Content: {}", path, content);
    }

    let parsed_value = content.parse::<f32>().unwrap_or_default();

    (parsed_value / max * 100.0) as i32
}

use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::{
    command::Command,
    config::{CommonConfig, DisplayMethod},
};

pub fn common_watcher(
    get_value: impl Fn() -> i32,
    config: &CommonConfig,
    tx: Sender<(Command, DisplayMethod)>,
) {
    let mut previous = get_value();

    loop {
        let current = get_value();

        if current != previous {
            let command = Command::new(config.tag.clone(), current);

            previous = current.clone();
            tx.send((command, config.display_with)).ok();
        }
        thread::sleep(Duration::from_millis(config.interval));
    }
}

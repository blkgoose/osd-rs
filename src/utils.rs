use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::{command::Command, config::DisplayMethod};

pub fn common_watcher(
    get_value: impl Fn() -> i32,
    kind: &crate::config::Kind,
    tx: Sender<(Command, DisplayMethod)>,
    interval: u64,
    display_with: DisplayMethod,
) {
    let mut previous = get_value();

    loop {
        let current = get_value();

        if current != previous {
            let command = if current > previous {
                Command::up(kind, current)
            } else {
                Command::down(kind, current)
            };

            previous = current.clone();
            tx.send((command, display_with)).ok();
        }
        thread::sleep(Duration::from_millis(interval));
    }
}

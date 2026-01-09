use std::thread;
use std::time::Duration;

use crate::{
    command::{Command, CommandSender},
    config::CommonConfig,
};

pub fn common_watcher(get_value: impl Fn() -> i32, config: &CommonConfig, tx: CommandSender) {
    let mut previous = get_value();

    loop {
        let current = get_value();
        if config.debug {
            println!(
                "Current value: {}, Previous value: {}, Tag: {}",
                current, previous, config.tag
            );
        }

        if current != previous {
            let command = Command::new(config.tag.clone(), current);

            previous = current;
            tx.send((command, config.display_with)).ok();
        }
        thread::sleep(Duration::from_millis(config.interval));
    }
}

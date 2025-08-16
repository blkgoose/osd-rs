use notify_rust::{Hint, Notification, NotificationHandle};
use std::{env, sync::mpsc};

use crate::{
    command::Command,
    config::{
        DisplayMethod::{self, NotifySend},
        Watcher,
    },
};

mod command;
mod config;
mod file_watcher;
mod poll_watcher;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <config.toml>", args[0]);
        std::process::exit(1);
    }

    let config = config::Config::from_file(&args[1]).expect("Failed to load config");
    let (tx, rx): (
        mpsc::Sender<(Command, DisplayMethod)>,
        mpsc::Receiver<(Command, DisplayMethod)>,
    ) = mpsc::channel();

    for watcher in config.watchers {
        match watcher {
            Watcher::File(file_config) => {
                file_watcher::watch(file_config.clone(), tx.clone());
            }
            Watcher::Poll(poll_config) => {
                poll_watcher::watch(poll_config.clone(), tx.clone());
            }
        }
    }

    let mut handle: Option<NotificationHandle> = None;
    for (command, display_method) in rx {
        match display_method {
            NotifySend => {
                let mut notification = match command {
                    Command::BrightnessUp(val) | Command::BrightnessDown(val) => {
                        Notification::new()
                            .appname("brightness")
                            .summary(&format!("brightness: {val}"))
                            .hint(Hint::CustomInt("value".to_string(), val))
                            .hint(Hint::Custom("osd-rs".to_string(), "brightness".to_string()))
                            .finalize()
                    }
                    Command::VolumeUp(val) | Command::VolumeDown(val) => Notification::new()
                        .appname("volume")
                        .summary(&format!("volume: {val}"))
                        .hint(Hint::CustomInt("value".to_string(), val))
                        .hint(Hint::Custom("osd-rs".to_string(), "volume".to_string()))
                        .finalize(),
                };

                match handle {
                    Some(ref mut n) => {
                        notification
                            .id(n.id())
                            .show()
                            .expect("Failed to update notification");
                    }
                    None => {
                        handle = Some(notification.show().expect("Failed to show notification"));
                    }
                }
            }
        }
    }
}

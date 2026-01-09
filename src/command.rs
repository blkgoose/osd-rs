use std::sync::mpsc;

use crate::config::DisplayMethod;

pub struct Command {
    pub tag: String,
    pub value: i32,
}

impl Command {
    pub(crate) fn new(tag: String, value: i32) -> Self {
        Self { tag, value }
    }
}

pub type CommandSender = mpsc::Sender<(Command, DisplayMethod)>;
pub type CommandReceiver = mpsc::Receiver<(Command, DisplayMethod)>;

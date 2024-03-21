use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    name: String,
    webhook_link: String,
    message_link: Option<String>,
    role_ping_id: Option<usize>,
    watch_folder: PathBuf,
    color: usize,
}

impl Config {
    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    pub fn webhook_link<'a>(&'a self) -> &'a str {
        &self.webhook_link
    }

    pub fn message_link<'a>(&'a self) -> Option<&'a str> {
        self.message_link.as_deref()
    }

    pub fn role_ping_id<'a>(&'a self) -> Option<usize> {
        self.role_ping_id
    }

    pub fn color<'a>(&'a self) -> usize {
        self.color
    }

    pub fn watch_folder<'a>(&'a self) -> &'a PathBuf {
        &self.watch_folder
    }
}
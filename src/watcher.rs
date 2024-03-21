use std::{ffi::OsString, path::PathBuf};

use directories::ProjectDirs;

use crate::{config::Config, webhook::{Embed, Field, Footer, Webhook}};

#[derive(Clone, Debug)]
pub struct Watcher {
    config: Config,
    data_dir: PathBuf,
    no_webhook: bool,
    directories: Vec<OsString>,
}

impl Watcher {
    pub fn new(config_path: &PathBuf, no_webhook: bool) -> Watcher {

        let config_str = std::fs::read_to_string(config_path).expect("Failed to read config");
        let config: Config = serde_json::from_str(&config_str).expect("Failed to serialize config");

        if !config.watch_folder().exists() {
            panic!("Watch folder does not exist!");
        }

        if let Some(proj_dirs) = ProjectDirs::from("xyz", "superyu", "nav1truenas") {
            let data_dir = proj_dirs.data_dir().to_path_buf();

            let mut directories = Vec::new();

            if let Ok(dirs_str) = std::fs::read_to_string(data_dir.join(format!("{}.json", config.name()))) {
                directories = serde_json::from_str(&dirs_str).expect("Failed to serialize directory cache");
            }

            return Watcher {
                config,
                data_dir,
                directories,
                no_webhook
            }
        }

        panic!("Could not get data directory!");
    }

    fn persist_directories(&self) {
        let persist_directory = self.data_dir.join(format!("{}.json", self.config.name()));

        if !self.data_dir.exists() {
            std::fs::create_dir_all(&self.data_dir).expect("Failed to create data directory");
        }

        let directories_str = serde_json::to_string(&self.directories).expect("Failed to serialize directories");
        std::fs::write(persist_directory, directories_str).expect("Failed to write directory cache");
    }

    /// Updates directory cache, returns the name of newly added directories.
    pub fn update_directories(&mut self) -> Vec<OsString> {
        let mut new_list = Vec::new();
        let mut newly_added = Vec::new();

        for dir_res in std::fs::read_dir(self.config.watch_folder()).expect("Failed to read directory") {
            if let Ok(dir) = dir_res {
                if dir.file_type().expect("Failed to get filetype").is_dir() {
                    let name = dir.file_name();

                    if !self.directories.contains(&name) {
                        newly_added.push(name.clone());
                    }

                    new_list.push(name);
                }
            }
        }

        self.directories = new_list;
        self.persist_directories();

        newly_added
    }

    fn clean_title<'a>(title: &'a str) -> String {
        let str = title.to_owned();

        let re = regex::bytes::Regex::new(r"(\s\[[a-zA-Z]+-\d+\])|(\s\[nAV1])").unwrap();
        let cleaned = re.replace_all(str.as_bytes(), "".as_bytes());
        let cleaned_string = String::from_utf8(cleaned.to_vec()).expect("Failed to construct String from bytes");

        cleaned_string
    }

    fn fire_webhook<'a>(&self, title: &'a str) {
        let reqwest = reqwest::blocking::Client::new();

        let content = {
            if let Some(role) = self.config.role_ping_id() {
                format!("<@&{role}> New upload!")
            } else {
                format!("New upload!")
            }
        };

        let fields = {
            let mut fields = Vec::new();

            if let Some(link) = self.config.message_link() {
                fields.push(
                    Field::builder(
                        String::from("Download"), 
                        link.to_owned()
                    ).build()
                )
            }

            fields
        };

        let webhook = Webhook::builder()
            .content(content)
            .username(
                String::from(
                    "nAV1"
                )
            )
            .avatar_url(
                String::from(
                    "https://cdn.discordapp.com/avatars/378993911609425920/ec1be00c3b542352f85c8a56d246c429?size=1024"
                )
            )
            .embeds(
                vec![
                    Embed::builder()
                        .title(Watcher::clean_title(title))
                        .color(self.config.color())
                        .fields(fields)
                        .footer(
                            Footer::builder(
                                String::from(
                                    "superyu"
                                )
                            )
                            .icon_url(
                                String::from(
                                    "https://cdn.discordapp.com/avatars/378993911609425920/ec1be00c3b542352f85c8a56d246c429?size=64"
                                )
                            )
                            .build()
                        ).build()
                ]
            )
            .build();

        reqwest.post(self.config.webhook_link())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(webhook.to_json())
            .send().expect("Failed to send");
    }

    pub fn run(&mut self) {
        let newly_added = self.update_directories();

        if !self.no_webhook {
            for title in newly_added {
                if let Some(str) = title.to_str() {
                    self.fire_webhook(str)
                }
            }
        }
    }
}

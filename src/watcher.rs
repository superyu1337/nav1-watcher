use std::{collections::HashMap, fs::read_dir, path::PathBuf};

use directories::ProjectDirs;

use crate::{config::Config, show::{Show, ShowComparison}, webhook::{Embed, Field, Footer, Webhook}};

#[derive(Clone, Debug)]
pub struct Watcher {
    config: Config,
    data_dir: PathBuf,
    no_webhook: bool,
    /// String is the CLEANED title!
    shows: HashMap<String, Show>,
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

            let mut shows = HashMap::new();

            if let Ok(dirs_str) = std::fs::read_to_string(data_dir.join(format!("{}.json", config.name()))) {
                shows = serde_json::from_str(&dirs_str).expect("Failed to serialize shows cache");
            }

            return Watcher {
                config,
                data_dir,
                shows,
                no_webhook
            }
        }

        panic!("Could not get data directory!");
    }

    fn persist_shows(&self) {
        let persist_directory = self.data_dir.join(format!("{}.json", self.config.name()));

        if !self.data_dir.exists() {
            std::fs::create_dir_all(&self.data_dir).expect("Failed to create data directory");
        }

        let shows_str = serde_json::to_string(&self.shows).expect("Failed to serialize directories");
        std::fs::write(persist_directory, shows_str).expect("Failed to write shows cache");
    }

    pub fn read_new(&mut self) -> HashMap<String, Show> {
        let mut new = HashMap::new();

        for dir_res in read_dir(self.config.watch_folder()).expect("Fild to read watch directory") {
            if let Ok(dir) = dir_res {
                if dir.file_type().expect("Failed to get file type").is_dir() {
                    let show = Show::from_folder(&dir.path());
                    new.insert(show.title().to_owned(), show);
                }
            }
        }

        new
    }

    pub fn update_shows(&mut self) -> Vec<Comparison> {
        let new = self.read_new();

        let mut comparisons = new
            .clone()
            .into_iter()
            .map(|(k, v)| {
                if let Some(other) = self.shows.get(&k) {
                    v.compare(other)
                } else {
                    None
                }
            })
            .filter_map(|f| f)
            .map(|f| Comparison::Changed(f))
            .collect::<Vec<Comparison>>();

        let mut new_shows = new
            .clone()
            .into_iter()
            .filter(|(k, _)| !self.shows.contains_key(k))
            .map(|(_, v)| Comparison::NewlyAdded(v))
            .collect::<Vec<Comparison>>();

        self.shows = new;
        self.persist_shows();

        comparisons.append(&mut new_shows);
        comparisons
    }

    fn fire_webhook<'a>(&self, comp: Comparison) {
        let reqwest = reqwest::blocking::Client::new();

        let content = {
            let msg = match &comp {
                Comparison::NewlyAdded(_) => {
                    format!("New show!")
                },
                Comparison::Changed(showcomp) => {
                    match showcomp {
                        ShowComparison::NewSeasons(_, seasons) => {
                            if seasons.len() > 1 {
                                format!("New seasons!")
                            } else {
                                format!("New season!")
                            }
                        },
                        ShowComparison::NewEpisodes(_, episodes) => {
                            if *episodes > 1 {
                                format!("New episodes!")
                            } else {
                                format!("New episode!")
                            }
                        },
                        
                    }
                },
            };

            if let Some(role) = self.config.role_ping_id() {
                format!("<@&{role}> {msg}")
            } else {
                format!("{msg}")
            }
        };

        let fields = {
            let mut fields = Vec::new();

            if let Some(link) = self.config.message_link() {
                fields.push(
                    Field::builder(
                        String::from("Download"), 
                        link.to_owned()
                    ).inline(true).build()
                )
            }

            match &comp {
                Comparison::NewlyAdded(show) => {
                    fields.push(
                        Field::builder(
                            String::from("Episodes"),
                            format!("{}", show.episode_count())
                        ).inline(true).build()
                    )
                },
                Comparison::Changed(showcomp) => {
                    match showcomp {
                        ShowComparison::NewSeasons(_, seasons) => {
                            let new_episodes = seasons
                                .iter()
                                .fold(0usize, |mut c, s| {
                                    c += s.episodes();
                                    c
                                });

                            fields.push(
                                Field::builder(
                                    String::from("New Episodes"),
                                    format!("{}", new_episodes)
                                ).inline(true).build()
                            );
                        },
                        ShowComparison::NewEpisodes(_, episodes) => {
                            if *episodes > 1 {
                                fields.push(
                                    Field::builder(
                                        String::from("New Episodes"),
                                        format!("{}", episodes)
                                    ).inline(true).build()
                                );
                            }
                        },
                    }
                },
            }

            fields
        };

        let title = {
            match &comp {
                Comparison::NewlyAdded(show) => format!("{}", show.title()),
                Comparison::Changed(showcomp) => {
                    match showcomp {
                        ShowComparison::NewSeasons(show, seasons) => {
                            if seasons.len() == 1 {
                                let first = seasons.first().expect("Failed to get first element");
                                format!(
                                    "{} (Season {})", 
                                    show.title(), 
                                    first.season_number()
                                )
                            } else {
                                let first = seasons.first().expect("Failed to get first element");
                                let last = seasons.last().expect("Failed to get first element");
                                format!(
                                    "{} (Seasons {}-{})",
                                    show.title(),
                                    first.season_number(),
                                    last.season_number()
                                )
                            }
                        },
                        ShowComparison::NewEpisodes(show, episodes) => {
                            if *episodes > 1 {
                                format!(
                                    "{} (+{} Episodes)",
                                    show.title(),
                                    episodes
                                )
                            } else {
                                format!(
                                    "{} (+{} Episode)",
                                    show.title(),
                                    episodes
                                )
                            }
                        },
                    }
                },
            }
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
                        .title(title)
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
        let comparisons = self.update_shows();

        if !self.no_webhook {
            for comp in comparisons {
                self.fire_webhook(comp);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Comparison {
    NewlyAdded(Show),
    Changed(ShowComparison),
}
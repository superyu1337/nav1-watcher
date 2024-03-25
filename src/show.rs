#![allow(dead_code)]

use std::{ffi::OsString, path::Path};

use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Show {
    folder_name: OsString,
    title: String,
    seasons: Vec<Season>,
}

impl Show {
    pub fn title<'a>(&'a self) -> &'a str {
        &self.title
    }

    pub fn folder_name<'a>(&'a self) -> &'a str {
        self.folder_name.to_str().expect("Failed to get &str")
    }

    pub fn episode_count(&self) -> usize {
        self.seasons.iter().fold(0usize, |mut counter, season| {
            counter += season.episodes();
            counter
        })
    }

    /// Compare two shows
    /// Self is the newest Show object
    /// other is the previous/persisted Show object
    pub fn compare2<'a>(&self, other: &Show) -> Vec<ShowComparison> {
        let new_seasons = self.seasons
            .clone()
            .into_iter()
            .filter(|this| {
                let other_opt = other.seasons
                    .iter()
                    .find(|f| f.season_number == this.season_number);

                match other_opt {
                    Some(_) => { false },
                    None => { true }
                }
            })
            .collect::<Vec<Season>>();

        let old_episodes = other.episode_count();
        let new_episodes = self.episode_count();
        /*
        let new_episodes = self.seasons
            .clone()
            .into_iter()
            .filter(|this| {
                let other_opt = other.seasons
                    .iter()
                    .find(|f| f.season_number == this.season_number);

                match other_opt {
                    Some(_) => { true },
                    None => { false }
                }
            })
            .fold(0usize, |mut c, s| {
                c += s.episodes;
                c
            });
        */

        let mut comps = Vec::new();

        if !new_seasons.is_empty() {
            comps.push(ShowComparison::NewSeasons(self.clone(), new_seasons));
        }

        if old_episodes < new_episodes {
            comps.push(ShowComparison::NewEpisodes(self.clone(), new_episodes-old_episodes));
        }

        comps
    }

    /// Compare two shows
    /// Self is the newest Show object
    /// other is the previous/persisted Show object
    pub fn compare<'a>(&self, other: &Show) -> Option<ShowComparison> {
        let new_seasons = self.seasons
            .clone()
            .into_iter()
            .filter(|this| {
                let other_opt = other.seasons
                    .iter()
                    .find(|f| f.season_number == this.season_number);

                match other_opt {
                    Some(_) => { false },
                    None => { true }
                }
            })
            .collect::<Vec<Season>>();

        let old_episodes = other.episode_count();
        let new_episodes = self.episode_count();

        if !new_seasons.is_empty() {
            return Some(ShowComparison::NewSeasons(self.clone(), new_seasons));
        }

        if old_episodes < new_episodes {
            return Some(ShowComparison::NewEpisodes(self.clone(), new_episodes-old_episodes));
        }

        None
    }

    pub fn from_folder(path: &Path) -> Show {
        let folder_name = path
            .file_name()
            .expect("Failed to get show folder name")
            .to_owned();

        let title = Show::clean_title(
            folder_name.to_str().expect("Failed to get &str from title")
        );

        let seasons = path.read_dir().expect("Failed to readdir show folder")
            .map(|f| f.ok())
            .fold(Vec::new(), |mut vec, f| {
                if let Some(entry) = f {
                    if entry.file_type().expect("Failed to get file type when fetching seasons").is_dir() {
                        if let Some(folder_name) = entry.file_name().to_str() {
                            if Season::is_season(folder_name) {
                                vec.push(
                                    Season::from_folder(&entry.path())
                                );
                            }
                        }
                    }
                }

                vec
            });

        Show {
            folder_name,
            title,
            seasons,
        }
    }

    fn clean_title<'a>(title: &'a str) -> String {
        let str = title.to_owned();

        let re = regex::bytes::Regex::new(r"(\s\[[a-zA-Z]+-\d+\])|(\s\[nAV1])|(\s\[nAV1-[0-9]+p-HDR])|(\s\[nAV1-[0-9]+p])").unwrap();
        let cleaned = re.replace_all(str.as_bytes(), "".as_bytes());
        let cleaned_string = String::from_utf8(cleaned.to_vec()).expect("Failed to construct String from bytes");

        cleaned_string
    }
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Season {
    folder_name: OsString,
    name: String,
    season_number: usize,
    episodes: usize,
}

impl Season {
    pub fn is_season(folder_name: &str) -> bool {
        let re = regex::bytes::Regex::new(r"^Season ([1-9]|[0-9][1-9]|[1-9]0)$").unwrap();
        re.captures(folder_name.as_bytes()).is_some()
    }

    pub fn from_folder(path: &Path) -> Season {
        let folder_name = path
            .file_name()
            .expect("Failed to get season folder name")
            .to_owned();

        let name = folder_name
            .to_str()
            .expect("Failed to get &str from folder name")
            .to_owned();

        let season_number = name
            .clone()
            .split_off(7)
            .parse::<usize>()
            .expect("Failed to parse season number");
    
        let episodes = path.read_dir().expect("Failed to readdir season folder")
            .map(|f| f.ok())
            .fold(0usize, |mut counter, f| {

                if let Some(entry) = f {
                    let is_file = entry.file_type().expect("Failed to get file type when fetching episodes").is_file();

                    if is_file && entry.file_name().to_string_lossy().ends_with(".mkv") {
                        counter += 1;
                    }
                }

                counter
            });

        Season { folder_name, name, season_number, episodes }
    }

    pub fn folder_name<'a>(&'a self) -> &'a str {
        self.folder_name.to_str().expect("Failed to get &str")
    }

    pub fn episodes(&self) -> usize {
        self.episodes
    }

    pub fn season_number(&self) -> usize {
        self.season_number
    }
}

#[derive(Clone, Debug)]
pub enum ShowComparison {
    NewEpisodes(Show, usize),
    NewSeasons(Show, Vec<Season>)
}
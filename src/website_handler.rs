use crate::generation_templates::{LandingTemplate, RepoListItem};
use crate::{
    http::{Method, Request, Response, StatusCode},
    server::Handler,
};
use askama::Template;
use core::panic;
use std::borrow::{BorrowMut, Cow};
use std::fs::{self, ReadDir};
use std::io::{Error, Read};
use std::path::PathBuf;
use toml::{Table, toml};

const MINUTE_SECONDS: u64 = 60;
const HOUR_SECONDS: u64 = 3_600;
const DAY_SECONDS: u64 = 86_400;
const WEEK_SECONDS: u64 = 604_800;
const MONTH_SECONDS: u64 = 2_628_003;
const YEAR_SECONDS: u64 = 31_536_000;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    return fs::read_to_string(path).ok();
                }

                println!("Directory traversal attack attempted: {}", file_path);
                None
            }

            Err(_) => None,
        }
    }

    fn calculate_modified_time(&mut self, time: u64) -> String {
        match time {
            t if t < MINUTE_SECONDS => format!("{} second(s) ago", time), // max 1 minute
            t if t < HOUR_SECONDS => format!("{} minute(s) ago", time / MINUTE_SECONDS), // max 1 hour
            t if t < DAY_SECONDS => format!("{} hour(s) ago", time / HOUR_SECONDS), // max 1 day
            t if t < WEEK_SECONDS => format!("{} day(s) ago", time / DAY_SECONDS),  // max 1 week
            t if t < MONTH_SECONDS => format!("{} week(s) ago", time / WEEK_SECONDS), // max 1 month
            t if t < YEAR_SECONDS => format!("{} month(s) ago", time / MONTH_SECONDS), // max 1 year
            _ => format!("{} year(s) ago", (time / YEAR_SECONDS)),
        }
    }

    fn handle_config(&mut self, repo_path: PathBuf, mut owner: String) -> () {}

    fn generate_landing_page(&mut self) -> Result<String, Error> {
        let repo_path = format!("{}/repos", env!("CARGO_MANIFEST_DIR"));
        let mut repo_list = vec![];

        for entry in fs::read_dir(repo_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name();

            if path.ends_with(".DS_Store") {
                continue;
            }

            // calculate modified time
            let metadata = fs::metadata(&path);
            let last_modified =
                self.calculate_modified_time(metadata?.modified()?.elapsed().unwrap().as_secs());

            let mut owner = String::new();
            let mut description = String::new();
            for repo_entry in fs::read_dir(path.join(".git"))? {
                println!("{:?} -> {:?}", path.file_name().unwrap(), repo_entry);
                let repo_entry = repo_entry.unwrap();
                let repo_path = repo_entry.path();
                let file_name = repo_path.file_name().unwrap().to_str().unwrap();
                match file_name {
                    "config" => {
                        if let Ok(file_contents) = fs::read_to_string(repo_path) {
                            let lines = file_contents.lines();
                            for line in lines {
                                let clean_line = line.trim_start();
                                if !clean_line.starts_with("url") {
                                    continue;
                                }
                                let parts = clean_line.split_once("https://").unwrap();
                                let mut path_parts = parts.1.split("/");
                                let _platform = path_parts.next().unwrap();
                                owner = String::from(path_parts.next().unwrap());
                                let _repo = path_parts.next().unwrap();
                            }
                        }
                    }
                    "description" => {
                        if let Ok(file_contents) = fs::read_to_string(repo_path) {
                            description = file_contents;
                        }
                    }
                    _ => {}
                }
            }

            let item = RepoListItem {
                path: String::from(path.to_str().unwrap()),
                name: String::from(file_name.unwrap().to_str().unwrap()),
                description,
                author: String::from(owner),
                last_edited: last_modified,
            };
            repo_list.push(item);
        }

        let landing_html = LandingTemplate { files: repo_list };
        Ok(landing_html.render().unwrap())
    }

    fn generate_other_pages(&mut self, path: &str) -> Option<String> {
        // TODO: aggrigate repos sub page details
        // 1. get repo name from route
        // 2. generate html page using askama based on sub route requested?
        // 3. TODO: cache file somewhere
        self.read_file(path)
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, Some(self.generate_landing_page().unwrap())),
                path => match self.generate_other_pages(path) {
                    Some(file) => Response::new(StatusCode::OK, Some(file)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            Method::POST => todo!(),
            Method::OPTIONS => todo!(),
            Method::PUT => todo!(),
            Method::DELETE => todo!(),
            Method::HEAD => todo!(),
            Method::CONNECT => todo!(),
            Method::TRACE => todo!(),
            Method::PATCH => todo!(),
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

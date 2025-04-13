use crate::{
    generation_templates::{AccountAboutTemplate, AccountIndexTemplate, RepoListItemTemplate},
    utils::calculate_modified_time,
};
use askama::Template;
use std::{fs, io::Error};

fn get_repo_path() -> String {
    format!("{}/repos", env!("CARGO_MANIFEST_DIR"))
}

pub(crate) fn generate_about_page() -> Result<String, askama::Error> {
    AccountAboutTemplate {
        description: vec![
            "Welcome to my git mirror/thingy mabob. Not exactly sure what im gonna do with this thing yet but i like building my own shit",
            "This is another line, idk how its gonna work?"
        ]
    }
    .render()
}

pub(crate) fn generate_landing_page() -> Result<String, Error> {
    let repo_path = get_repo_path();
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
            calculate_modified_time(metadata?.modified()?.elapsed().unwrap().as_secs());

        let mut owner = String::new();
        let mut description = String::new();
        for repo_entry in fs::read_dir(path.join(".git"))? {
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

        let item = RepoListItemTemplate {
            path: String::from(path.to_str().unwrap()),
            name: String::from(file_name.unwrap().to_str().unwrap()),
            description,
            author: String::from(owner),
            last_edited: last_modified,
        };
        repo_list.push(item);
    }

    let landing_html = AccountIndexTemplate { files: repo_list };
    Ok(landing_html.render().unwrap())
}

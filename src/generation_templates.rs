use askama::Template;

use crate::utils::FileTreeLayer;

#[derive(Template)]
#[template(path = "account_index.html", ext = "html")]
pub struct AccountIndexTemplate {
    pub files: Vec<RepoListItemTemplate>,
}

#[derive(Template)]
#[template(path = "account_about.html", ext = "html")]
pub struct AccountAboutTemplate<'a> {
    pub description: Vec<&'a str>,
}

#[derive(Template)]
#[template(path = "repo_list_item.html", ext = "html")]
pub struct RepoListItemTemplate {
    pub path: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub last_edited: String,
}

#[derive(Template)]
#[template(path = "file_tree.html", ext = "html")]
pub struct FileTreeTemplate {
    pub file_layer: FileTreeLayer,
}

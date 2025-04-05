use askama::Template;

#[derive(Template)]
#[template(path = "repos.html", ext = "html")]
pub struct LandingTemplate {
    pub files: Vec<RepoListItem>,
}

#[derive(Template)]
#[template(path = "repo_list_item.html", ext = "html")]
pub struct RepoListItem {
    pub path: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub last_edited: String,
}

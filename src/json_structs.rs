// Structs for deserializing github api calls

#[derive(RustcDecodable, Debug, Clone)]
pub struct RepoOwner {
    pub login: Option<String>,
    pub id: i64,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    //pub type_name: Option<String>,
    pub site_admin: bool,
}

#[derive(RustcDecodable, Debug, Clone)]
pub struct Repo {
    pub id: i64,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub owner: RepoOwner,
    pub private: bool,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: bool,
    pub url: Option<String>,
    pub forks_url: Option<String>,
    pub keys_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub teams_url: Option<String>,
    pub hooks_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub events_url: Option<String>,
    pub assignees_url: Option<String>,
    pub branches_url: Option<String>,
    pub tags_url: Option<String>,
    pub blobs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub trees_url: Option<String>,
    pub statuses_url: Option<String>,
    pub languages_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub contributors_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub commits_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub comments_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub contents_url: Option<String>,
    pub compare_url: Option<String>,
    pub merges_url: Option<String>,
    pub archive_url: Option<String>,
    pub downloads_url: Option<String>,
    pub issues_url: Option<String>,
    pub pulls_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub labels_url: Option<String>,
    pub releases_url: Option<String>,
    pub deployments_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub pushed_at: Option<String>,
    pub git_url: Option<String>,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: Option<String>,
    pub has_issues: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: i64,
    pub mirror_url: Option<String>,
    pub open_issues_count: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub default_branch: Option<String>,
}

#[derive(RustcDecodable, Debug, Clone)]
pub struct User {
    pub login: Option<String>,
    pub id: i64,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    //pub type_name: Option<String>,
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<String>,
    pub bio: Option<String>,
    pub public_repos: i64,
    pub public_gists: i64,
    pub followers: i64,
    pub following: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
use std::collections::HashMap;
use hyper::Client;
use hyper::header::UserAgent;
use std::io::prelude::*;
use json_structs::{User, Repo};
use rustc_serialize::json;

header! { (Authorization, "Authorization") => [String] }

// Get languages for a repository. return hashmap with those languages.
// Since langs are unknown, this can't return a struct
pub fn get_repo_languages(languages_url: String) -> HashMap<String, i64> {
    let client = Client::new();
    let mut resp = client.get(languages_url.as_str())
                         .header(UserAgent("rust-hyper".to_string()))
                         .header(Authorization("token f4f6caa6a3e5f5da491e42d6ee14708f325ad655".to_string()))
                         .send()
                         .unwrap();
    let mut response_content = String::new();
    match resp.read_to_string(&mut response_content) {
        Ok(_) => {
            ;
        }
        Err(err) => panic!("{}", err),
    }

    let mut langs = HashMap::new();

    //remove brackets
    response_content.pop();
    response_content.remove(0);

    // This is fragile; small changes to the api could break this.
    for line in response_content.split(",") {
        if line == "" {
            continue;
        }

        let json_row: Vec<&str> = line.split(":").collect();
        assert!(json_row.len() == 2);

        let mut key = json_row[0].to_string();
        let value = json_row[1].to_string().parse::<i64>().unwrap();

        //remove quotes
        key.remove(0);
        key.pop();

        langs.insert(key, value);
    }
    return langs;
}


// Get all repos from a user. Returns vector of Repo structs, each containing
// info on one repo and api links to more info on that repo.
pub fn get_repos(user: User) -> Vec<Repo> {
    let client = Client::new();
    let mut resp = client.get(user.repos_url.unwrap().as_str())
                         .header(UserAgent("rust-hyper".to_string()))
                         .header(Authorization("token f4f6caa6a3e5f5da491e42d6ee14708f325ad655".to_string()))
                         .send()
                         .unwrap();
    let mut response_content = String::new();
    match resp.read_to_string(&mut response_content) {
        Ok(_) => {
            ;
        }
        Err(err) => panic!("{}", err),
    }
    let repos: Vec<Repo> = json::decode(&*response_content).unwrap();
    return repos;
}


// Get user struct from login name. Struct contains some info and many useful
// api links to more info.
pub fn get_user(username: &str) -> User {
    let url = format!("https://api.github.com/users/{}", username);
    let client = Client::new();
    let mut resp = client.get(url.as_str())
                         .header(UserAgent("rust-hyper".to_string()))
                         .header(Authorization("token f4f6caa6a3e5f5da491e42d6ee14708f325ad655".to_string()))
                         .send()
                         .unwrap();
    let mut response_content = String::new();
    match resp.read_to_string(&mut response_content) {
        Ok(_) => {
            ;
        }
        Err(err) => panic!("{}", err),
    }
    let user: User = json::decode(&*response_content).unwrap();
    return user;
}
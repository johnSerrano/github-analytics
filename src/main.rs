extern crate hyper;
extern crate rustc_serialize;

mod json_structs;

use hyper::Client;
use hyper::header::UserAgent;
use std::io::prelude::*;
use json_structs::{User, Repo};
use rustc_serialize::json;

fn main() {
    let user = get_user("johnserrano");

    println!("USER: {}\n", &user.clone().login.unwrap());

    let repos = get_repos(user);
    assert!(repos.len() > 1);

    println!("***** REPOS *****");
    for repo in repos {
        let name = &repo.clone().name.unwrap();
        println!("{}", name);
    }
}

fn get_repos(user: User) -> Vec<Repo> {
    let client = Client::new();
    let mut resp = client.get(user.repos_url.unwrap().as_str())
                         .header(UserAgent("rust-hyper".to_string()))
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

fn get_user(username: &str) -> User {
    let url = format!("https://api.github.com/users/{}", username);
    let client = Client::new();
    let mut resp = client.get(url.as_str())
                         .header(UserAgent("rust-hyper".to_string()))
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
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
    print_report(user);
}


// Compile all stats and print report
fn print_report(user: User) {
    // Print user name
    let user_clone = user.clone();
    println!("USER: {}\n", user_clone.login.unwrap());

    let repos = get_repos(user);

    // Print total number of repositories
    println!("Total repositories: {}\n", user_clone.public_repos);

    // Print all repos and info on each
    if repos.len() > 1 {
        println!("***** REPOS *****");
        for repo in repos {
            let name = &repo.clone().name.unwrap();
            println!("{}", name);
        }
    }
}


// Get all repos from a user. Returns vector of Repo structs, each containing
// info on one repo and api links to more info on that repo.
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


// Get user struct from login name. Struct contains some info and many useful
// api links to more info.
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

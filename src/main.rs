#[macro_use]
extern crate hyper;
extern crate rustc_serialize;

mod json_structs;
mod parsers;

use parsers::{get_user, get_repos, get_languages, get_langs_for_user};
use json_structs::User;
use std::env;


fn main() {
    assert!(env::args().len() > 1);
    let args: Vec<_> = env::args().collect();
    let username = args[1].to_string();
    let user = get_user(username.as_str());
    print_report(user);
}


// Compile all stats and print report
fn print_report(user: User) {
    // We borrowed user so we should make sure to give it back like new!
    let user_clone = user.clone();

    let repos = get_repos(user);
    let repo_lang_map = get_languages(repos.clone());
    let user_lang_map = get_langs_for_user(repo_lang_map.clone());

    // Print user name
    println!("USER: {}\n", user_clone.login.unwrap());

    // Print stats on user
    println!("Total public repositories: {}", user_clone.public_repos);
    println!("Total public gists: {}", user_clone.public_gists);
    println!("Followers: {}", user_clone.followers);
    println!("Following: {}", user_clone.following);
    println!("\nLanguages:");
    for (lang, freq) in &user_lang_map {
        println!("{}: {}", lang, freq);
    }

    println!("\n");

    // Print all repos and info on each
    if repos.len() > 1 {
        println!("***** REPOS *****");
        for repo in repos {
            let name = repo.name.unwrap();
            println!("{}", name);

            let langs = repo_lang_map[&repo.full_name.unwrap()].clone();
            for (lang, freq) in &langs {
                println!("{}: {}", lang, freq);
            }

            println!("\n");
        }
    }
}

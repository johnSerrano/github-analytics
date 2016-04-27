#[macro_use] extern crate hyper;
extern crate rustc_serialize;

mod json_structs;
mod parsers;

use parsers::{get_user, get_repos, get_repo_languages};
use json_structs::User;


fn main() {
    let user = get_user("johnserrano");
    print_report(user);
}


// Compile all stats and print report
fn print_report(user: User) {
    // We borrowed user so we should make sure to give it back like new!
    let user_clone = user.clone();

    let repos = get_repos(user);

    // Print user name
    println!("USER: {}\n", user_clone.login.unwrap());

    // Print stats on user
    println!("Total public repositories: {}", user_clone.public_repos);
    println!("Total public gists: {}", user_clone.public_gists);
    println!("Followers: {}", user_clone.followers);
    println!("Following: {}", user_clone.following);

    println!("\n");

    // Print all repos and info on each
    if repos.len() > 1 {
        println!("***** REPOS *****");
        for repo in repos {
            let name = repo.name.unwrap();
            println!("{}", name);

            let langs = get_repo_languages(repo.languages_url.unwrap());
            println!("{:?}", langs);

            println!("\n");
        }
    }
}

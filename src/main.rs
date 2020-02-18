use std::{ process };

extern crate git2;
use git2::{ Error, ErrorCode, Repository };

fn get_branch_name(repo: &Repository) -> Result<String, Error> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return Err(e),
    };
    let head = head.as_ref().and_then(|h| h.shorthand());
    return Ok(head.unwrap_or("HEAD (no branch)").to_string());
}

fn main () {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(_e) => {
            process::exit(0);
        }
    };
    println!("{}", get_branch_name(&repo).unwrap());
}

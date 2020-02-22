use std::{ env, process };
use std::path::Path;
use std::fmt::Write;

extern crate gethostname;
use gethostname::gethostname;

extern crate git2;
use git2::{ Error, ErrorCode, Repository, StatusOptions };


fn get_branch_name(repo: &Repository) -> Result<String, Error> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return Err(e),
    };
    let head = head.as_ref().and_then(|h| h.shorthand());
    // TODO: detached HEAD (e.g. in a checkout of HEAD~1)
    return Ok(head.unwrap_or("(empty branch)").to_string());
}

fn index_changed(repo: &Repository) -> bool {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    // TODO: might want to recurse_untracked_dirs

    let statuses = repo.statuses(Some(&mut opts)).unwrap();
    for entry in statuses.iter() {
        let is_index_changed = match entry.status() {
            s if s.contains(git2::Status::INDEX_NEW) => true,
            s if s.contains(git2::Status::INDEX_MODIFIED) => true,
            s if s.contains(git2::Status::INDEX_DELETED) => true,
            s if s.contains(git2::Status::INDEX_RENAMED) => true,
            s if s.contains(git2::Status::INDEX_TYPECHANGE) => true,
            s if s.contains(git2::Status::WT_NEW) => true,
            s if s.contains(git2::Status::WT_MODIFIED) => true,
            s if s.contains(git2::Status::WT_DELETED) => true,
            s if s.contains(git2::Status::WT_RENAMED) => true,
            s if s.contains(git2::Status::WT_TYPECHANGE) => true,
            _ => false,
        };
        if is_index_changed {
            return true;
        }
    }
    return false;
}

fn main () {
    let user = match env::var("USER") {
        Ok(val) => val,
        Err(_e) => String::new(),
    };

    // TODO: if it matches HOME, then print it as ~
    // TODO: less unwrap and actually think about handling errors? unwrap_or, unwrap_or_else
    let pwd = env::current_dir().unwrap();
    let wd = pwd.as_path().file_name().unwrap().to_str().unwrap();

    let _hostname = gethostname();  // im bad at rust why do i have to separate this
    let hostname = _hostname.to_str().unwrap();

    let mut prompt = String::new();
    write!(&mut prompt, "{}@{} {}", user, hostname, wd).unwrap();

    // XXX: need to canonicalize otherwise .parent will return Some("")
    // see https://github.com/rust-lang/rust/issues/36861
    let _gitroot = Path::new(".").canonicalize().unwrap();
    let mut gitroot = _gitroot.as_path();
    loop {
        if gitroot.join(".git").is_dir() {
            break;
        }
        gitroot = match gitroot.parent() {
            Some(p) => p,
            None => {
                println!("{} $ ", prompt.to_string());
                process::exit(0);
            }
        };
    }

    let repo = match Repository::open(gitroot) {
        Ok(repo) => repo,
        Err(_e) => {
            println!("{} $ ", prompt.to_string());
            process::exit(0);
        }
    };

    if index_changed(&repo) {
        prompt.push_str(" *");
    }

    write!(&mut prompt, " {}", get_branch_name(&repo).unwrap()).unwrap();
    println!("{} $ ", prompt.to_string());
}

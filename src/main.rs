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
    // colorization is optional and read through these env vars (recommended to populate via tput)
    // it would be far too expensive to parse terminfo databases
    let style_user = env::var("PROMPT_STYLE_USER").unwrap_or_default();
    let style_hostname = env::var("PROMPT_STYLE_HOSTNAME").unwrap_or_default();
    let style_wd = env::var("PROMPT_STYLE_WD").unwrap_or_default();
    let style_branch = env::var("PROMPT_STYLE_BRANCH").unwrap_or_default();
    let style_reset = env::var("PROMPT_STYLE_RESET").unwrap_or_default();

    let user = env::var("USER").unwrap_or_default();

    // TODO: if it matches HOME, then print it as ~
    let pwd = env::current_dir().unwrap();
    let wd = pwd.as_path().file_name().unwrap().to_str().unwrap();

    let _hostname = gethostname();  // im bad at rust why do i have to separate this
    let hostname = _hostname.to_str().unwrap();

    let mut prompt = String::new();
    write!(
        &mut prompt, "{}{}{}@{}{}{} {}{}{}",
        style_user, user, style_reset,
        style_hostname, hostname, style_reset,
        style_wd, wd, style_reset,
    ).unwrap();

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

    let mut branch_name = get_branch_name(&repo).unwrap();
    // XXX: this isn't 100% correct, but it'll do. I don't want to additionally inspect symbolic ref.
    if branch_name == "HEAD" {
        branch_name = "(detached HEAD)".to_string()
    }
    write!(&mut prompt, " {}{}{}", style_branch, branch_name, style_reset).unwrap();
    println!("{} $ ", prompt.to_string());
}

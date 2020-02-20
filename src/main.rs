use std::{ env, process };

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
    // Also, write tests based on the relatively new git branch --show-current,
    // which in an empty repo will be master but we want to show HEAD (no branch)
    // and also in detached head we want to show (detached HEAD) instead of blank (whcih is show-current's behavior)
    return Ok(head.unwrap_or("HEAD (no branch)").to_string());
}

fn index_changed(repo: &Repository) -> bool {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);  // recurse_untracked_dirs?

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
    print!("{}@{} {}", user, hostname, wd);

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(_e) => {
            print!("\n");
            process::exit(0);
        }
    };

    if index_changed(&repo) {
        print!(" *");
    }

    println!(" {}", get_branch_name(&repo).unwrap());
}

use std::{ env, process };

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
    // TODO: conditionally try HOSTNAME based on SHELL (set by bash iirc)
    // HOST is zsh-specific
    // Also, on linux procfs you can read from /proc/sys/kernel/hostname.
    // So might be worth trying that first and then falling back to shell env vars.
    // The most correct + universal (posix) way is to use the gethostname syscall,
    // and there's a crate for that, but i'd like to say reading env vars are faster
    // since they don't involve syscalls. Definitely better than procfs though.
    let hostname = match env::var("HOST") {
        Ok(val) => val,
        Err(_e) => String::new(),
    };

    print!("{}@{}", user, hostname);

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(_e) => {
            print!("\n");
            process::exit(0);
        }
    };

    if index_changed(&repo) {
        print!(" * ");
    }

    println!("{}", get_branch_name(&repo).unwrap());
}

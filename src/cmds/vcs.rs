/*
IAY | Minimalist prompt for Bash/Zsh!
Copyright (C) 2021 Aaqa Ishtyaq
*/
use git2::{Oid, Repository, Status, StatusOptions};
use lazy_static::lazy_static;
use nu_ansi_term::Color::*;
use std::cell::Cell;
use std::env;
use std::path::Path;

// Taken from: https://github.com/Ryooooooga/almel/blob/467e8f699e840c418a7eed5e0e22cf9c34ed1dca/src/segments/git_repo.rs
lazy_static! {
    static ref STATUS_CONFLICTED: Status = Status::CONFLICTED;
    static ref STATUS_UNSTAGED: Status =
        Status::WT_MODIFIED | Status::WT_DELETED | Status::WT_RENAMED | Status::WT_TYPECHANGE;
    static ref STATUS_STAGED: Status = Status::INDEX_NEW
        | Status::INDEX_MODIFIED
        | Status::INDEX_DELETED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE;
    static ref STATUS_MODIFIED: Status = Status::INDEX_MODIFIED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE
        | Status::WT_MODIFIED
        | Status::WT_RENAMED
        | Status::WT_TYPECHANGE;
    static ref STATUS_NEW: Status = Status::WT_NEW;
    static ref STATUS_DELETED: Status = Status::WT_DELETED | Status::INDEX_DELETED;
}

fn vcs_status() -> Option<(String, String)> {
    let current_dir = env::var("PWD").unwrap();

    let mut repo: Option<Repository> = None;
    let current_path = Path::new(&current_dir[..]);
    for path in current_path.ancestors() {
        if let Ok(r) = Repository::open(path) {
            repo = Some(r);
            break;
        }
    }

    // return if not a git repository
    repo.as_ref()?;

    let mut repo = repo.unwrap();

    let mut commit_dist: String = "".into();
    if let Some((ahead, behind)) = get_ahead_behind(&repo) {
        if ahead > 0 {
            commit_dist.push_str(Magenta.bold().paint(&format!(" {}⇡", ahead)).as_str());
        }
        if behind > 0 {
            commit_dist.push_str(Cyan.bold().paint(&format!(" {}⇣", behind)).as_str());
        }
    }

    let (repo_stat, branch_color_deduced) = build_git_status_tray(&mut repo);

    let branch_color = branch_color_deduced;
    //let commit_color = "Magenta";

    let reference = match repo.head() {
        Ok(r) => r,
        Err(_) => return None,
    };

    let branch = if reference.is_branch() {
        match branch_color.as_str() {
            "Green" => Green
                .bold()
                .paint(&format!(
                    "{}{}",
                    reference.shorthand().unwrap(),
                    commit_dist
                ))
                .to_string(),
            "Yellow" => Yellow
                .bold()
                .paint(&format!(
                    "{}{}",
                    reference.shorthand().unwrap(),
                    commit_dist
                ))
                .to_string(),
            "Blue" => Blue
                .bold()
                .paint(&format!(
                    "{}{}",
                    reference.shorthand().unwrap(),
                    commit_dist
                ))
                .to_string(),
            _ => Green
                .bold()
                .paint(&format!(
                    "{}{}",
                    reference.shorthand().unwrap(),
                    commit_dist
                ))
                .to_string(),
        }
    } else {
        let commit = reference.peel_to_commit().unwrap();
        let id = commit.id();
        Magenta
            .bold()
            .paint(&format!("{:.6}{}", id, commit_dist))
            .to_string()
    };

    let mut vcs_stat = String::new();
    if repo_stat.chars().count() >= 1 {
        let open_pair = match branch_color.as_str() {
            "Green" => Green.bold().paint(" ["),
            "Yellow" => Yellow.bold().paint(" ["),
            "Blue" => Blue.bold().paint(" ["),
            _ => Green.bold().paint(" ["),
        }
        .to_string();
        let close_pair = match branch_color.as_str() {
            "Green" => Green.bold().paint("]"),
            "Yellow" => Yellow.bold().paint("]"),
            "Blue" => Blue.bold().paint("]"),
            _ => Green.bold().paint("]"),
        }
        .to_string();
        vcs_stat = [open_pair, repo_stat, close_pair].concat()
    }

    Some((branch, vcs_stat))
}

fn build_git_status_tray(repo: &mut Repository) -> (String, String) {
    //let git_clean_color = "green";
    //let git_wt_added_color = "yellow";
    //let git_index_modified_color = "green";
    //let git_wt_modified_color = "red";
    //let git_branch_modified_color = "blue";
    let mut repo_stat = String::new();
    let mut branch_color_deduced = "Green";

    let file_stats = get_repo_statuses(repo);

    if file_stats.intersects(*STATUS_NEW) {
        let stat_symbol = "!";
        branch_color_deduced = "Yellow";
        repo_stat += Yellow.bold().paint(&*stat_symbol).as_str();
    }

    if file_stats.intersects(*STATUS_UNSTAGED) {
        let stat_symbol = "±";
        branch_color_deduced = "Blue";
        repo_stat += Red.bold().paint(&*stat_symbol).as_str();
    }

    if file_stats.intersects(*STATUS_STAGED) {
        let stat_symbol = "±";
        branch_color_deduced = "Blue";
        repo_stat += Green.bold().paint(&*stat_symbol).as_str();
    }

    if is_stashed(repo) {
        let stat_symbol = "$";
        match branch_color_deduced {
            "Yellow" => repo_stat += Yellow.bold().paint(&*stat_symbol).as_str(),
            "Blue" => repo_stat += Blue.bold().paint(&*stat_symbol).as_str(),
            "Green" => repo_stat += Green.bold().paint(&*stat_symbol).as_str(),
            _ => repo_stat += Green.bold().paint(&*stat_symbol).as_str(),
        }
    }

    (repo_stat, branch_color_deduced.to_string())
}

fn is_stashed(repo: &mut Repository) -> bool {
    let stashed = Cell::new(false);

    let _ = repo.stash_foreach(|_a: usize, _b: &str, _c: &Oid| -> bool {
        stashed.set(true);
        // stop as soon as we determine that there's any stash
        false
    });

    stashed.get()
}

fn get_repo_statuses(repo: &Repository) -> Status {
    let mut options = StatusOptions::new();
    options.include_untracked(true);

    repo.statuses(Some(&mut options))
        .map(|statuses| statuses.iter().fold(Status::empty(), |a, b| a | b.status()))
        .unwrap_or_else(|_| Status::empty())
}

fn get_ahead_behind(r: &Repository) -> Option<(usize, usize)> {
    let head = (r.head().ok())?;
    if !head.is_branch() {
        return None;
    }

    let head_name = (head.shorthand())?;
    let head_branch = (r.find_branch(head_name, git2::BranchType::Local).ok())?;
    let upstream = (head_branch.upstream().ok())?;
    let head_oid = (head.target())?;
    let upstream_oid = (upstream.get().target())?;

    r.graph_ahead_behind(head_oid, upstream_oid).ok()
}

fn vcs_tray() -> String {
    let vcs_tuple = vcs_status();
    let mut vcs_component = String::new();
    if let Some((branch, status)) = vcs_tuple {
        vcs_component = format!(" {}{} ", branch, status);
    } else {
        vcs_component.push(' ');
    }

    vcs_component
}

pub fn status() -> String {
    vcs_tray()
}

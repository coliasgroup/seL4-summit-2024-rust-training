use std::{collections::BTreeMap, ops::Bound};
use std::path::Path;
use std::ops::RangeBounds;
use std::str;

use git2::{Commit, Repository};

const PATH: &str = "../..";

fn main() {
    let top = Repository::init(PATH).unwrap();
    let repo = top.find_submodule("code").unwrap().open().unwrap();
    let head = repo
        .find_commit(repo.refname_to_id("HEAD").unwrap())
        .unwrap();
    let steps = Steps::new(&repo, head);
    let s = steps.fragment("1.A", "workspaces/root-task/hello/src/main.rs", 3..=14);
    println!("{}", s);
    println!("{}", steps.commit_hash("1.A"));
}

struct Steps<'a> {
    repo: &'a Repository,
    commits: BTreeMap<String, Commit<'a>>,
}

impl<'a> Steps<'a> {
    fn new(repo: &'a Repository, tip: Commit<'a>) -> Self {
        let mut commits = BTreeMap::new();
        let mut commit = tip;
        loop {
            let summary = commit.summary().unwrap();
            commits.insert(summary.to_owned(), commit.clone());
            println!("{:?}", summary);
            if summary == "0" {
                break;
            }
            assert_eq!(commit.parent_count(), 1);
            commit = commit.parent(0).unwrap();
        }
        Self { repo, commits }
    }

    fn commit_hash(&self, step: &str) -> String {
        format!("{}", self.commits[step].id())
    }

    fn fragment(&self, step: &str, path: impl AsRef<Path>, bounds: impl RangeBounds<usize>) -> String {
        let a = self.commits[step]
            .tree()
            .unwrap()
            .get_path(path.as_ref())
            .unwrap()
            .to_object(self.repo)
            .unwrap();
        let b = a
            .peel_to_blob()
            .unwrap();
        let c = b
            .content();
        let content = c;
        let s = str::from_utf8(content).unwrap();
        let start = match bounds.start_bound() {
            Bound::Included(i) => *i - 1,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => 0,
        };
        let end = match bounds.end_bound() {
            Bound::Included(i) => Some(*i),
            Bound::Excluded(i) => Some(*i - 1),
            Bound::Unbounded => None,
        };
        let it = s.lines().skip(start);
        let mut s = String::new();
        match end {
            Some(end) => {
                for line in it.take(end - start) {
                    s.push_str(line);
                    s.push_str("\n");
                }
            }
            None => {
                for line in it {
                    s.push_str(line);
                    s.push_str("\n");
                }
            }
        }
        s
    }
}

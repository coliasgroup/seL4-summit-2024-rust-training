use std::ops::RangeBounds;
use std::path::Path;
use std::str;
use std::{collections::BTreeMap, ops::Bound};

use either::Either;
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
    steps: BTreeMap<String, Commit<'a>>,
}

impl<'a> Steps<'a> {
    fn new(repo: &'a Repository, last_step: Commit<'a>) -> Self {
        let mut steps = BTreeMap::new();
        let mut commit = last_step;
        loop {
            let summary = commit.summary().unwrap();
            steps.insert(summary.to_owned(), commit.clone());
            if summary == "0" {
                break;
            }
            assert_eq!(commit.parent_count(), 1);
            commit = commit.parent(0).unwrap();
        }
        Self { repo, steps }
    }

    fn commit_hash(&self, step: &str) -> String {
        format!("{}", self.steps[step].id())
    }

    fn fragment(
        &self,
        step: &str,
        path: impl AsRef<Path>,
        bounds: impl RangeBounds<usize>,
    ) -> String {
        let blob = self.steps[step]
            .tree()
            .unwrap()
            .get_path(path.as_ref())
            .unwrap()
            .to_object(self.repo)
            .unwrap()
            .peel_to_blob()
            .unwrap();
        let s = str::from_utf8(blob.content()).unwrap();
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
        let it = s.lines();
        let it = match end {
            Some(end) => Either::Right(it.take(end)),
            None => Either::Left(it),
        }
        .into_iter();
        it.skip(start).flat_map(|line| [line, "\n"]).collect()
    }
}

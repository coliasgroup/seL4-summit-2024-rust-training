use std::ops::RangeBounds;
use std::path::Path;
use std::str;
use std::{collections::BTreeMap, ops::Bound};

use regex::Regex;
use either::Either;
use git2::{Repository, Oid};

pub struct Steps {
    repo: Repository,
    steps: BTreeMap<String, Oid>,
}

const CODE_SUBMODULE_PATH: &str = "code";

impl Steps {
    pub fn new_at(top_level_repo_path: impl AsRef<Path>) -> Self {
        let top_level_repo = Repository::init(top_level_repo_path.as_ref()).unwrap();
        let code_repo = top_level_repo.find_submodule(CODE_SUBMODULE_PATH).unwrap().open().unwrap();
        let head = code_repo
            .find_commit(code_repo.refname_to_id("HEAD").unwrap())
            .unwrap()
            .id();
        Self::new(code_repo, head)
    }

    pub fn new(repo: Repository, last_step: Oid) -> Self {
        let summary_re = Regex::new(r"^[0-9]+\.[A-Z]$").unwrap();
        let mut steps = BTreeMap::new();
        let mut commit_id = last_step;
        loop {
            let commit = repo.find_commit(commit_id).unwrap();
            let summary = commit.summary().unwrap();
            steps.insert(summary.to_owned(), commit.id());
            if summary == "0" {
                break;
            }
            assert!(summary_re.is_match(summary));
            assert_eq!(commit.parent_count(), 1);
            commit_id = commit.parent(0).unwrap().id();
        }
        Self { repo, steps }
    }

    pub fn commit_hash(&self, step: &str) -> String {
        format!("{}", self.steps[step])
    }

    pub fn fragment(
        &self,
        step: &str,
        path: impl AsRef<Path>,
        bounds: impl RangeBounds<usize>,
    ) -> String {
        let blob = self.repo.find_commit(self.steps[step]).unwrap()
            .tree()
            .unwrap()
            .get_path(path.as_ref())
            .unwrap()
            .to_object(&self.repo)
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

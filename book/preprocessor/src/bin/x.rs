use git2::Repository;

const PATH: &str = "../..";

fn main() {
    let top = Repository::init(PATH).unwrap();
    let repo = top.find_submodule("code").unwrap().open().unwrap();
    let head = repo.find_commit(repo.refname_to_id("HEAD").unwrap()).unwrap();
    let mut commit = head;
    while commit.summary().unwrap() != "0" {
        println!("{:?}", commit.summary().unwrap());
        assert_eq!(commit.parent_count(), 1);
        commit = commit.parent(0).unwrap();
    }
}

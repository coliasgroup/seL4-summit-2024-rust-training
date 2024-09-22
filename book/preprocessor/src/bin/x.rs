use mdbook_sel4_training::Steps;

const PATH: &str = "../..";

fn main() {
    let steps = Steps::new_at(PATH);
    let s = steps.fragment("1.A", "workspaces/root-task/hello/src/main.rs", 3..=14);
    println!("{}", s);
    println!("{}", steps.commit_hash("1.A"));
}

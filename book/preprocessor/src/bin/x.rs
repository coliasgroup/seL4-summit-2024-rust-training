use mdbook_sel4_rust_training::Steps;

fn main() {
    let steps = Steps::new_simple("../..", "HEAD");
    let s = steps.fragment("1.A", "workspaces/root-task/hello/src/main.rs", 3..=14);
    println!("{}", s);
    println!("{}", steps.commit_hash("1.A"));
}

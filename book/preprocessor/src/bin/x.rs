use mdbook_sel4_rust_training::{Step, Steps};

fn main() {
    let steps = Steps::new_simple("../..", "HEAD");
    let s = steps.fragment(
        &Step::parse("1.A"),
        "workspaces/root-task/hello/src/main.rs",
        3..=14,
    );
    println!("{}", s);
    println!("{}", steps.commit_hash(&Step::parse("1.A")));
}

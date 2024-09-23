use clap::{Command, Arg};

use mdbook_sel4_rust_training::Steps;

fn main() {
    let matches = Command::new("")
        .arg(
            Arg::new("top_level_dir")
                .value_name("TOP_LEVEL_DIR")
                .required(true)
        )
        .arg(
            Arg::new("rev")
                .long("rev")
                .short('r')
                .value_name("REV")
                .default_value("HEAD")
        )
        .get_matches();

    let top_level_dir = matches.get_one::<String>("top_level_dir").unwrap();
    let rev = matches.get_one::<String>("rev").unwrap();
    
    let steps = Steps::new_simple(top_level_dir, rev);

    for (_step, id) in steps.iter() {
        println!("{id}");
    }
}

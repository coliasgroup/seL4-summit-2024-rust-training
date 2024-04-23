//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use clap::{Arg, ArgMatches, Command};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use semver::{Version, VersionReq};
use std::io;
use std::env;
use std::process;
use std::path::{PathBuf, Path};
use std::fmt::Write;
use regex::{Regex, Captures};

pub fn make_app() -> Command {
    Command::new("")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let local_root = env::var("FOO_LOCAL_ROOT").unwrap();
    let remote_prefix = env::var("FOO_REMOTE_PREFIX").unwrap();

    let preprocessor = This::new(local_root, remote_prefix);

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

pub struct This {
    local_root: PathBuf,
    remote_prefix: String,
}

impl This {
    pub fn new(local_root: impl AsRef<Path>, remote_prefix: impl AsRef<str>) -> This {
        This {
            local_root: local_root.as_ref().to_owned(),
            remote_prefix: remote_prefix.as_ref().to_owned(),
        }
    }
}

impl Preprocessor for This {
    fn name(&self) -> &str {
        "this preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(ref mut ch) = *section {
                let r = Regex::new("\\{\\{\\s*#foo\\s+\"(?<attrs>.*)\"\\s+(?<path>.*):(?<start>\\d+):(?<end>\\d+)\\s*\\}\\}").unwrap();
                ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                    let attrs = captures.name("attrs").unwrap().as_str();
                    let path = captures.name("path").unwrap().as_str();
                    let start = captures.name("start").unwrap().as_str();
                    let end = captures.name("end").unwrap().as_str();
                    let abs_path = format!("{}", self.local_root.join(path).display());
                    let url = format!("{}/{path}#L{start}-L{end}", self.remote_prefix);
                    let mut s = String::new();

                    writeln!(&mut s, "<div class=\"foo\">").unwrap();

                    writeln!(&mut s, "<div class=\"foo-header\">").unwrap();
                    // write!(&mut s, "<pre><code class=\"hljs\">").unwrap();
                    write!(&mut s, "<pre><code>").unwrap();
                    // write!(&mut s, "<p>").unwrap();
                    write!(&mut s, "<a href=\"{url}\">{path}:{start}:{end}</a>").unwrap();
                    // write!(&mut s, "</p>").unwrap();
                    write!(&mut s, "</code></pre>").unwrap();
                    writeln!(&mut s, "").unwrap();
                    writeln!(&mut s, "</div>").unwrap();

                    // write!(&mut s, "<pre><code class=\"hljs\">").unwrap();
                    // write!(&mut s, "<a href=\"{url}\">{path}:{start}:{end}</a>").unwrap();
                    // write!(&mut s, "</code></pre>").unwrap();
                    // writeln!(&mut s, "").unwrap();

                    writeln!(&mut s, "<div class=\"foo-body\">").unwrap();

                    writeln!(&mut s, "").unwrap();

                    writeln!(&mut s, "```{attrs}").unwrap();
                    writeln!(&mut s, "{{{{#include {abs_path}:{start}:{end}}}}}").unwrap();
                    writeln!(&mut s, "```").unwrap();

                    writeln!(&mut s, "").unwrap();

                    writeln!(&mut s, "</div>").unwrap();

                    // writeln!(&mut s, "<div class=\"foo-header\">").unwrap();
                    // // write!(&mut s, "<pre><code class=\"hljs\">").unwrap();
                    // write!(&mut s, "<pre><code>").unwrap();
                    // // write!(&mut s, "<p>").unwrap();
                    // write!(&mut s, "<a href=\"{url}\">{path}:{start}:{end}</a>").unwrap();
                    // // write!(&mut s, "</p>").unwrap();
                    // write!(&mut s, "</code></pre>").unwrap();
                    // writeln!(&mut s, "").unwrap();
                    // writeln!(&mut s, "</div>").unwrap();

                    // writeln!(&mut s, "[{path}]({url})").unwrap();

                    // write!(&mut s, "<pre style=\"text-align: right\"><code class=\"hljs\">").unwrap();
                    // write!(&mut s, "<a href=\"{url}\">{path}:{start}:{end}</a>").unwrap();
                    // write!(&mut s, "</code></pre>").unwrap();
                    // writeln!(&mut s, "").unwrap();

                    writeln!(&mut s, "</div>").unwrap();

                    s
                }).into_owned();
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

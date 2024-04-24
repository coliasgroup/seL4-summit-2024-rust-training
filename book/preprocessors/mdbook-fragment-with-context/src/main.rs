//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use clap::{Arg, Command};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use regex::{Captures, Regex};
use semver::{Version, VersionReq};
use std::env;
use std::fmt::Write;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let matches = Command::new("")
        .subcommand(Command::new("supports").arg(Arg::new("renderer").required(true)))
        .get_matches();

    let local_root = env::var("MDBOOK_FRAGMENT_WITH_CONTEXT_LOCAL_ROOT").unwrap();
    let remote_prefix = env::var("MDBOOK_FRAGMENT_WITH_CONTEXT_REMOTE_PREFIX").unwrap();

    let preprocessor = This::new(local_root, remote_prefix);

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        let renderer = sub_args.get_one::<String>("renderer").unwrap();
        assert!(preprocessor.supports_renderer(renderer));
    } else {
        handle_preprocessing(&preprocessor).unwrap();
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

    fn render(&self, attrs: &str, path: &str, start: &str, end: &str) -> String {
        let abs_path = format!("{}", self.local_root.join(path).display());
        let url = format!("{}/{path}#L{start}-L{end}", self.remote_prefix);

        let mut s = String::new();

        writeln!(&mut s, "<div class=\"fragment-with-context\">").unwrap();

        writeln!(&mut s, "<div class=\"fragment-with-context-context\">").unwrap();
        write!(&mut s, "<pre><code>").unwrap();
        write!(&mut s, "<a href=\"{url}\">{path}:{start}:{end}</a>").unwrap();
        write!(&mut s, "</code></pre>").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "</div>").unwrap();

        writeln!(&mut s, "<div class=\"fragment-with-context-fragment\">").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "```{attrs}").unwrap();
        writeln!(&mut s, "{{{{#include {abs_path}:{start}:{end}}}}}").unwrap();
        writeln!(&mut s, "```").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "</div>").unwrap();

        writeln!(&mut s, "</div>").unwrap();

        s
    }
}

impl Preprocessor for This {
    fn name(&self) -> &str {
        "fragment-with-context"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(ref mut ch) = *section {
                let r = Regex::new("\\{\\{\\s*#fragement_with_context\\s+\"(?<attrs>.*)\"\\s+(?<path>.*):(?<start>\\d+):(?<end>\\d+)\\s*\\}\\}").unwrap();
                ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                    self.render(
                        captures.name("attrs").unwrap().as_str(),
                        captures.name("path").unwrap().as_str(),
                        captures.name("start").unwrap().as_str(),
                        captures.name("end").unwrap().as_str(),
                    )
                }).into_owned();
            }
        });

        Ok(book)
    }
}

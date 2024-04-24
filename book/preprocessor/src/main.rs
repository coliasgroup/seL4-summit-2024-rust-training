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
use std::path::PathBuf;

fn main() {
    let matches = Command::new("")
        .subcommand(Command::new("supports").arg(Arg::new("renderer").required(true)))
        .get_matches();

    let preprocessor = This {
        local_root: env::var("MDBOOK_GH_LINKS_LOCAL_ROOT").unwrap().into(),
        repo: env::var("MDBOOK_GH_LINKS_REPO").unwrap(),
        rev: env::var("MDBOOK_GH_LINKS_REV").unwrap(),
    };

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
    repo: String,
    rev: String,
}

impl This {
    fn url(&self, path: &str, is_directory: bool) -> String {
        format!(
            "https://github.com/{}/{}/{}/{path}",
            self.repo,
            if is_directory { "tree" } else { "blob" },
            self.rev
        )
    }

    fn render_fragment_with_context(
        &self,
        attrs: &str,
        hidden_path_part: &str,
        visible_path_part: &str,
        start: &str,
        end: &str,
    ) -> String {
        let path = format!("{hidden_path_part}{visible_path_part}");
        let local_path = self.local_root.join(&path).display().to_string();
        let url = format!("{}#L{start}-L{end}", self.url(&path, false));

        let mut s = String::new();

        writeln!(&mut s, "<div class=\"fragment-with-gh-link\">").unwrap();

        writeln!(&mut s, "<div class=\"fragment-with-gh-link-link\">").unwrap();
        write!(&mut s, "<pre><code>").unwrap();
        write!(
            &mut s,
            "<a href=\"{url}\">{visible_path_part}:{start}:{end}</a>"
        )
        .unwrap();
        write!(&mut s, "</code></pre>").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "</div>").unwrap();

        writeln!(&mut s, "<div class=\"fragment-with-gh-link-fragment\">").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "```{attrs}").unwrap();
        writeln!(&mut s, "{{{{#include {local_path}:{start}:{end}}}}}").unwrap();
        writeln!(&mut s, "```").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "</div>").unwrap();

        writeln!(&mut s, "</div>").unwrap();

        s
    }

    fn render_link(
        &self,
        hidden_path_part: &str,
        visible: &str,
        visible_path_part: &str,
        start: Option<&str>,
        end: Option<&str>,
    ) -> String {
        let path = format!("{hidden_path_part}{visible_path_part}");
        let local_path = self.local_root.join(&path);
        let url = {
            let mut s = self.url(&path, local_path.is_dir());
            if let Some(start) = start {
                write!(&mut s, "#L{start}").unwrap();
                if let Some(end) = end {
                    write!(&mut s, "-L{end}").unwrap();
                }
            }
            s
        };
        format!("[{visible}]({url})")
    }
}

impl Preprocessor for This {
    fn name(&self) -> &str {
        "gh-links"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(ref mut ch) = *section {
                {
                    let r = Regex::new("\\{\\{\\s*#fragment_with_gh_link\\s+\"(?<attrs>[^}]*)\"\\s+(\\((?<hidden_path_part>[^\\)]*)\\))?(?<visible_path_part>[^}]*):(?<start>\\d+):(?<end>\\d+)\\s*\\}\\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_fragment_with_context(
                            captures.name("attrs").unwrap().as_str(),
                            captures.name("hidden_path_part").map(|m| m.as_str()).unwrap_or(""),
                            captures.name("visible_path_part").unwrap().as_str(),
                            captures.name("start").unwrap().as_str(),
                            captures.name("end").unwrap().as_str(),
                        )
                    }).into_owned();
                }
                {
                    let r = Regex::new("\\{\\{\\s*#gh_link\\s+(\\((?<hidden_path_part>[^\\)]*)\\))?(?<visible>(?<visible_path_part>[^:}]*)(:(?<start>\\d+)(:(?<end>\\d+))?)?)\\s*\\}\\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_link(
                            captures.name("hidden_path_part").map(|m| m.as_str()).unwrap_or(""),
                            captures.name("visible").unwrap().as_str(),
                            captures.name("visible_path_part").unwrap().as_str(),
                            captures.name("start").map(|m| m.as_str()),
                            captures.name("end").map(|m| m.as_str()),
                        )
                    }).into_owned();
                }
            }
        });

        Ok(book)
    }
}

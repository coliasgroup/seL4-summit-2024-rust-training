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

struct This {
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

    fn render_fragment_with_context(&self, attrs: &str, link: &Link) -> String {
        let link_text = link.text();
        let range_suffix = link.range_suffix();
        let path = link.path();
        let local_path = self.local_root.join(&path).display().to_string();
        let url = format!("{}{}", self.url(&path, false), link.url_fragment());

        let mut s = String::new();

        writeln!(&mut s, "<div class=\"fragment-with-gh-link\">").unwrap();

        writeln!(&mut s, "<div class=\"fragment-with-gh-link-link\">").unwrap();
        write!(&mut s, "<pre><code>").unwrap();
        write!(&mut s, "<a href=\"{url}\">{link_text}</a>").unwrap();
        write!(&mut s, "</code></pre>").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "</div>").unwrap();

        writeln!(&mut s, "<div class=\"fragment-with-gh-link-fragment\">").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "```{attrs}").unwrap();
        writeln!(&mut s, "{{{{#include {local_path}{range_suffix}}}}}").unwrap();
        writeln!(&mut s, "```").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "</div>").unwrap();

        writeln!(&mut s, "</div>").unwrap();

        s
    }

    fn render_link(&self, link: &Link) -> String {
        let path = link.path();
        let local_path = self.local_root.join(&path);
        format!(
            "[{}]({}{})",
            link.text(),
            self.url(&path, local_path.is_dir()),
            link.url_fragment()
        )
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
                    let r = Regex::new("\\{\\{\\s*#fragment_with_gh_link\\s+\"(?<attrs>[^}]*)\"\\s+(?<link>.*?)\\s*\\}\\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_fragment_with_context(
                            captures.name("attrs").unwrap().as_str(),
                            &Link::parse(captures.name("link").unwrap().as_str()).unwrap(),
                        )
                    }).into_owned();
                }
                {
                    let r = Regex::new("\\{\\{\\s*#gh_link\\s+(?<link>.*?)\\s*\\}\\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_link(
                            &Link::parse(captures.name("link").unwrap().as_str()).unwrap(),
                        )
                    }).into_owned();
                }
            }
        });

        Ok(book)
    }
}

#[derive(Debug)]
struct Link {
    text: Option<String>,
    hidden_path_part: Option<String>,
    visible_path_part: String,
    start: Option<String>,
    end: Option<String>,
}

impl Link {
    fn parse(s: &str) -> Option<Link> {
        let r = Regex::new(
            r"(?x)
            ^
            (\[(?<text>[^\]]*)\]\s+)?
            (\((?<hidden_path_part>[^\)]*)\))?
            (?<visible_path_part>[^:]*)(:(?<start>\d+)(:(?<end>\d+))?)?
            $
        ",
        )
        .unwrap();
        r.captures(s).map(|captures| Self {
            text: captures.name("text").map(|m| m.as_str().to_owned()),
            hidden_path_part: captures
                .name("hidden_path_part")
                .map(|m| m.as_str().to_owned()),
            visible_path_part: captures
                .name("visible_path_part")
                .unwrap()
                .as_str()
                .to_owned(),
            start: captures.name("start").map(|m| m.as_str().to_owned()),
            end: captures.name("end").map(|m| m.as_str().to_owned()),
        })
    }

    fn path(&self) -> String {
        let mut s = String::new();
        if let Some(hidden_path_part) = &self.hidden_path_part {
            write!(&mut s, "{hidden_path_part}").unwrap();
        }
        write!(&mut s, "{}", self.visible_path_part).unwrap();
        s
    }

    fn text(&self) -> String {
        if let Some(text) = &self.text {
            text.to_owned()
        } else {
            self.visible()
        }
    }

    fn visible(&self) -> String {
        let mut s = String::new();
        write!(&mut s, "{}", self.visible_path_part).unwrap();
        write!(&mut s, "{}", self.range_suffix()).unwrap();
        s
    }

    fn range_suffix(&self) -> String {
        let mut s = String::new();
        if let Some(start) = &self.start {
            write!(&mut s, ":{start}").unwrap();
            if let Some(end) = &self.end {
                write!(&mut s, ":{end}").unwrap();
            }
        }
        s
    }

    fn url_fragment(&self) -> String {
        let mut s = String::new();
        if let Some(start) = &self.start {
            write!(&mut s, "#L{start}").unwrap();
            if let Some(end) = &self.end {
                write!(&mut s, "-L{end}").unwrap();
            }
        }
        s
    }
}

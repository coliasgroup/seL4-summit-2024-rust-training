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
        gh_link_local_root: env::var("MDBOOK_GH_LINK_LOCAL_ROOT").unwrap().into(),
        gh_link_repo: env::var("MDBOOK_GH_LINK_REPO").unwrap(),
        gh_link_rev: env::var("MDBOOK_GH_LINK_REV").unwrap(),
        manual_url: env::var("MDBOOK_MANUAL_URL").unwrap(),
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
    gh_link_local_root: PathBuf,
    gh_link_repo: String,
    gh_link_rev: String,
    manual_url: String,
}

impl This {
    fn render_fragment_with_gh_link(&self, attrs: &str, link: &GitHubLink) -> String {
        let link_text = link.text();
        let range_suffix = link.range_suffix();
        let local_path = self
            .gh_link_local_root
            .join(&link.path())
            .display()
            .to_string();
        let url = self.gh_link_url(link, false);

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

    fn render_gh_link(&self, link: &GitHubLink) -> String {
        let local_path = self.gh_link_local_root.join(link.path());
        format!(
            "[{}]({})",
            link.text(),
            self.gh_link_url(link, local_path.is_dir()),
        )
    }

    fn gh_link_url(&self, link: &GitHubLink, is_directory: bool) -> String {
        format!(
            "https://github.com/{}/{}/{}/{}",
            self.gh_link_repo,
            if is_directory { "tree" } else { "blob" },
            self.gh_link_rev,
            link.url_suffix(),
        )
    }

    fn render_manual_link(&self, link: &ManualLink) -> String {
        link.render(&self.manual_url)
    }
}

impl Preprocessor for This {
    fn name(&self) -> &str {
        "sel4-tutorial"
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
                        self.render_fragment_with_gh_link(
                            captures.name("attrs").unwrap().as_str(),
                            &GitHubLink::parse(captures.name("link").unwrap().as_str()).unwrap(),
                        )
                    }).into_owned();
                }
                {
                    let r = Regex::new(r"\{\{\s*#gh_link\s+(?<link>.*?)\s*\}\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_gh_link(
                            &GitHubLink::parse(captures.name("link").unwrap().as_str()).unwrap(),
                        )
                    }).into_owned();
                }
                {
                    let r = Regex::new(r"\{\{\s*#manual_link\s+(?<link>.*?)\s*\}\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_manual_link(
                            &ManualLink::parse(captures.name("link").unwrap().as_str()).unwrap(),
                        )
                    }).into_owned();
                }

            }
        });

        Ok(book)
    }
}

#[derive(Debug)]
struct GitHubLink {
    text: Option<String>,
    hidden_path_part: Option<String>,
    visible_path_part: String,
    start: Option<String>,
    end: Option<String>,
}

impl GitHubLink {
    fn parse(s: &str) -> Option<Self> {
        let r = Regex::new(
            r"(?x)
            ^
            (\[(?<text>.*?)\]\s+)?
            (\((?<hidden_path_part>.*?)\))?
            (?<visible_path_part>.*?)(:(?<start>\d+)(:(?<end>\d+))?)?
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

    fn url_suffix(&self) -> String {
        let mut s = self.path();
        if let Some(start) = &self.start {
            write!(&mut s, "#L{start}").unwrap();
            if let Some(end) = &self.end {
                write!(&mut s, "-L{end}").unwrap();
            }
        }
        s
    }
}

#[derive(Debug)]
struct ManualLink {
    text: Option<String>,
    section: Option<String>,
    section_name: Option<String>,
}

impl ManualLink {
    fn parse(s: &str) -> Option<Self> {
        let r = Regex::new(
            r"(?x)
            ^
            (
                \[
                    (?<text>.*?)
                \]
                (\s+|$)
            )?
            (
                \#
                (?<section>.*?)
                (\s+|$)
            )?
            (
                \(
                    (?<section_name>.*?)
                \)
                (\s+|$)
            )?
            $
        ",
        )
        .unwrap();
        r.captures(s).map(|captures| Self {
            text: captures.name("text").map(|m| m.as_str().to_owned()),
            section: captures.name("section").map(|m| m.as_str().to_owned()),
            section_name: captures.name("section_name").map(|m| m.as_str().to_owned()),
        })
    }

    fn render(&self, url: &str) -> String {
        let text = self.text.clone().unwrap_or_else(|| {
            let mut s = format!("seL4 Reference Manual");
            if let Some(section) = &self.section {
                write!(&mut s, " § {section}").unwrap();
                if let Some(section_name) = &self.section_name {
                    write!(&mut s, " ({section_name})").unwrap();
                }
            }
            s
        });
        let fragment = if let Some(section) = &self.section {
            let ty = if section.contains('.') {
                "section"
            } else {
                "chapter"
            };
            format!("#{ty}.{section}")
        } else {
            String::new()
        };
        format!("[{text}]({url}{fragment})")
    }
}

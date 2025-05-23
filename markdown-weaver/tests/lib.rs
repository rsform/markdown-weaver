#![cfg(feature = "html")]

use markdown_weaver::{Options, Parser};

#[rustfmt::skip]
mod suite;

#[inline(never)]
pub fn test_markdown_html(
    input: &str,
    output: &str,
    smart_punct: bool,
    metadata_blocks: bool,
    old_footnotes: bool,
    subscript: bool,
    wikilinks: bool,
    deflists: bool,
    obsidian: bool,
) {
    let mut s = String::new();

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_MATH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_SUPERSCRIPT);
    if wikilinks {
        opts.insert(Options::ENABLE_WIKILINKS);
    }
    if subscript {
        opts.insert(Options::ENABLE_SUBSCRIPT);
    }
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_GFM);
    if old_footnotes {
        opts.insert(Options::ENABLE_OLD_FOOTNOTES);
    } else {
        opts.insert(Options::ENABLE_FOOTNOTES);
    }
    if metadata_blocks {
        opts.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
        opts.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    }
    if smart_punct {
        opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    }
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    if deflists {
        opts.insert(Options::ENABLE_DEFINITION_LIST);
    }
    if obsidian {
        opts.insert(Options::ENABLE_OBSIDIAN_EMBEDS);
    }

    let p = Parser::new_ext(input, opts);
    markdown_weaver::html::push_html(&mut s, p);

    // normalizing the HTML using html5ever may hide actual errors
    // assert_eq!(html_standardize(output), html_standardize(&s));
    assert_eq!(html_standardize(output), html_standardize(&s));
}

fn html_standardize(s: &str) -> String {
    s.replace("<br>", "<br />")
        .replace("<br/>", "<br />")
        .replace("<hr>", "<hr />")
        .replace("<hr/>", "<hr />")
        // permit extra or missing line breaks only between tags
        .replace(">\n<", "><")
}

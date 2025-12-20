use super::test_markdown_html;

#[test]
fn test_basic() {
    let original = r##"![[dog.png {attr: value}]]
"##;

    let expected = r##"<p><img src="dog.png" attr="value" alt="dog.png {attr: value}" /></p>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, true, false, true,
    );
}

#[test]
fn test_multiple_attrs() {
    let original = r##"![[cat.png {.classy, attr1: value1, attr2: value2}]]
"##;
    let expected = r##"<p><img src="cat.png" class="classy" attr1="value1" attr2="value2" alt="cat.png {.classy, attr1: value1, attr2: value2}" /></p>
"##;
    test_markdown_html(
        original, expected, false, false, false, false, true, false, true,
    );
}

#[test]
fn test_weird_spacing_and_commas() {
    let original = r##"![[frog.png { one:1 ,two :2,  three :   3    }]]
"##;
    let expected = r##"<p><img src="frog.png" one="1" two="2" three="3" alt="frog.png { one:1 ,two :2,  three :   3    }" /></p>
"##;
    test_markdown_html(
        original, expected, false, false, false, false, true, false, true,
    );
}

#[test]
fn test_empty_and_broken_attrs() {
    let original = r##"![[error.png {, , : ,.junk, broken:   , :dud}]]
"##;
    // Only ".junk" (as class) should be parsed.
    let expected = r##"<p><img src="error.png" class="junk" alt="error.png {, , : ,.junk, broken:   , :dud}" /></p>
"##;
    test_markdown_html(
        original, expected, false, false, false, false, true, false, true,
    );
}

#[test]
fn test_trailing_comma_and_braces() {
    let original = r##"![[hat.png { foo: bar,   }]]
"##;
    let expected = r##"<p><img src="hat.png" foo="bar" alt="hat.png { foo: bar,   }" /></p>
"##;
    test_markdown_html(
        original, expected, false, false, false, false, true, false, true,
    );
}

#[test]
fn test_plain_markdown_image_with_attrs() {
    let original = r"![alt](cat.png {.fancy, attr: value})";
    // Standard markdown image with weaver attrs after
    let expected = r#"<p><img src="cat.png" class="fancy" attr="value" alt="alt" /></p>
"#;
    test_markdown_html(
        original, expected, false, false, false, false, true, false, true,
    );
}

#[test]
fn test_feature_flag_disabled() {
    let original = r##"![[dino.png {attr: val}]]
"##;
    // If obsidian_wikilinks/attrs disabled, attrs aren't parsed:
    let expected = r##"<p>![[dino.png {attr: val}]]</p>
"##;
    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

// Event-based tests for weaver blocks in various positions
use markdown_weaver::{Event, Options, Parser, Tag, TagEnd};

fn collect_events(input: &str) -> Vec<Event<'_>> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_WIKILINKS);
    opts.insert(Options::ENABLE_OBSIDIAN_EMBEDS);
    opts.insert(Options::ENABLE_MATH); // Required for { } to be scanned inline
    Parser::new_ext(input, opts).collect()
}

#[test]
fn test_block_level_before_paragraph_events() {
    let input = "{.intro, data-section: hero}\nThis is the intro paragraph.\n";
    let events: Vec<_> = collect_events(input);

    // Verify exact sequence
    let mut iter = events.iter();

    // WeaverBlock start with correct attrs
    let weaver_start = iter.next().expect("expected weaver block start");
    match weaver_start {
        Event::Start(Tag::WeaverBlock(_, attrs)) => {
            assert!(attrs.classes.contains(&"intro".into()));
            assert!(attrs.attrs.contains(&("data-section".into(), "hero".into())));
        }
        other => panic!("expected WeaverBlock start, got {:?}", other),
    }

    // WeaverBlock end
    assert!(matches!(iter.next(), Some(Event::End(TagEnd::WeaverBlock(_)))));

    // Paragraph start
    assert!(matches!(iter.next(), Some(Event::Start(Tag::Paragraph { .. }))));

    // Text content
    match iter.next() {
        Some(Event::Text(t)) => assert_eq!(t.as_ref(), "This is the intro paragraph."),
        other => panic!("expected text, got {:?}", other),
    }

    // Paragraph end
    assert!(matches!(iter.next(), Some(Event::End(TagEnd::Paragraph(_)))));
}

#[test]
fn test_inline_weaver_block_events() {
    let input = "Some text {.highlight} more text.\n";
    let events: Vec<_> = collect_events(input);

    let mut iter = events.iter();

    // Paragraph start
    assert!(matches!(iter.next(), Some(Event::Start(Tag::Paragraph { .. }))));

    // Text before weaver block
    match iter.next() {
        Some(Event::Text(t)) => assert_eq!(t.as_ref(), "Some text "),
        other => panic!("expected 'Some text ', got {:?}", other),
    }

    // WeaverBlock start
    match iter.next() {
        Some(Event::Start(Tag::WeaverBlock(_, attrs))) => {
            assert!(attrs.classes.contains(&"highlight".into()));
        }
        other => panic!("expected WeaverBlock start, got {:?}", other),
    }

    // WeaverBlock end
    assert!(matches!(iter.next(), Some(Event::End(TagEnd::WeaverBlock(_)))));

    // Text after weaver block
    match iter.next() {
        Some(Event::Text(t)) => assert_eq!(t.as_ref(), " more text."),
        other => panic!("expected ' more text.', got {:?}", other),
    }

    // Paragraph end
    assert!(matches!(iter.next(), Some(Event::End(TagEnd::Paragraph(_)))));
}

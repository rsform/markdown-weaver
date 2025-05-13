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
    let original = r##"![[cat.png {classy, attr1: value1, attr2: value2}]]
"##;
    let expected = r##"<p><img src="cat.png" class="classy" attr1="value1" attr2="value2" alt="cat.png {classy, attr1: value1, attr2: value2}" /></p>
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
    let original = r##"![[error.png {, , : ,junk, broken:   , :dud}]]
"##;
    // Only "junk" (as class) should be parsed.
    let expected = r##"<p><img src="error.png" class="junk" alt="error.png {, , : ,junk, broken:   , :dud}" /></p>
"##;
    test_markdown_html(
        original, expected, false, false, false, false, true, false, true,
    );
}

#[test]
fn test_trailing_comma_and_braces() {
    let original = r##"![[hat.png { foo: bar,   } ]]
"##;
    let expected = r##"<p><img src="hat.png" foo="bar" alt="hat.png { foo: bar,   }" /></p>
"##;
    test_markdown_html(
        original, expected, false, false, false, false, true, false, true,
    );
}

#[test]
fn test_plain_markdown_image_is_untouched() {
    let original = r"![alt](cat.png {attr: value})";
    // Standard markdown image, do NOT parse {attr: value}
    let expected = r#"<p><img src="cat.png {attr: value}" alt="alt" /></p>
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

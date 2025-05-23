// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn blockquotes_tags_test_1() {
    let original = r##"> This is a normal blockquote without tag.
"##;
    let expected = r##"<blockquote><p>This is a normal blockquote without tag.</p></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_2() {
    let original = r##"> [!NOTE]
> Note blockquote
"##;
    let expected = r##"<blockquote class="markdown-alert-note"><p>Note blockquote</p></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_3() {
    let original = r##"> [!TIP]
> Tip blockquote
"##;
    let expected = r##"<blockquote class="markdown-alert-tip"><p>Tip blockquote</p></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_4() {
    let original = r##"> [!IMPORTANT]
> Important blockquote
"##;
    let expected = r##"<blockquote class="markdown-alert-important"><p>Important blockquote</p></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_5() {
    let original = r##"> [!WARNING]
> Warning blockquote
"##;
    let expected = r##"<blockquote class="markdown-alert-warning"><p>Warning blockquote</p></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_6() {
    let original = r##"> [!CAUTION]
> Caution blockquote
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Caution blockquote</p></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_7() {
    let original = r##"> [!CAUTION]
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_8() {
    let original = r##"> [!CAUTION]
> Line 1.
> Line 2.
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Line 1.
Line 2.</p></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_9() {
    let original = r##"> [!CAUTION]
> Line 1.
> [!CAUTION]
> Line 2.
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Line 1.
[!CAUTION]
Line 2.</p></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_10() {
    let original = r##"> [!CAUTION]
> Line 1.
> > [!TIP]
> Line 2.
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Line 1.</p><blockquote class="markdown-alert-tip"><p>Line 2.</p></blockquote></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_11() {
    let original = r##"> [!CAUTION]
> Line 1.


> [!TIP]
> Line 2.
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Line 1.</p></blockquote><blockquote class="markdown-alert-tip"><p>Line 2.</p></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_12() {
    let original = r##"> > [!CAUTION]
> > Line 1.
> Line 2.
"##;
    let expected = r##"<blockquote><blockquote class="markdown-alert-caution"><p>Line 1.
Line 2.</p></blockquote></blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_13() {
    let original = r##"> [!CAUTION]
> Line 1.
> > [!NOTE]
> > Line 2.
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Line 1.</p>
<blockquote class="markdown-alert-note"><p>Line 2.</p></blockquote>
</blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_14() {
    let original = r##"> [!caution]
> Line 1.
> > [!note]
> > Line 2.
"##;
    let expected = r##"<blockquote class="markdown-alert-caution"><p>Line 1.</p>
<blockquote class="markdown-alert-note"><p>Line 2.</p></blockquote>
</blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_15() {
    let original = r##"  * loose lists

    > [!NOTE]
    > sink ships
"##;
    let expected = r##"<ul><li><p>loose lists</p>
<blockquote class="markdown-alert-note"><p>sink ships</p></blockquote>
</li></ul>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_16() {
    let original = r##"> [!NOTE]
sink ships

> [!NOTE]
> sink ships

  * loose lists

    > [!NOTE]
    sink ships

    > [!NOTE]
    > sink ships
"##;
    let expected = r##"<blockquote class="markdown-alert-note"><p>sink ships</p></blockquote>
<blockquote class="markdown-alert-note"><p>sink ships</p></blockquote>
<ul><li><p>loose lists</p>
<blockquote class="markdown-alert-note"><p>sink ships</p></blockquote>
<blockquote class="markdown-alert-note"><p>sink ships</p></blockquote>
</li></ul>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_17() {
    let original = r##"  * loose lists

    > [!NOTE]
    - sink ships

    > [!NOTE]
    > - sink ships
"##;
    let expected = r##"<ul><li><p>loose lists</p>
<blockquote class="markdown-alert-note"></blockquote>
<ul><li>sink ships</li></ul>
<blockquote class="markdown-alert-note">
<ul><li>sink ships</li></ul></blockquote>
</li></ul>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

#[test]
fn blockquotes_tags_test_18() {
    let original = r##"> [!Hello]
> This should be a normal block quote.
"##;
    let expected = r##"<blockquote>
<p>[!Hello]
This should be a normal block quote.</p>
</blockquote>
"##;

    test_markdown_html(
        original, expected, false, false, false, false, false, false, false,
    );
}

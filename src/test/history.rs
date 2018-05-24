//! History related commands tests
use super::assert_history;
use consts::KeyPress;

#[test]
fn down_key() {
    assert_history(&["line1"], &[KeyPress::Down, KeyPress::Enter], ("", ""));
    assert_history(
        &["line1", "line2"],
        &[KeyPress::Up, KeyPress::Up, KeyPress::Down, KeyPress::Enter],
        ("line2", ""),
    );
    assert_history(
        &["line1"],
        &[
            KeyPress::Char('a'),
            KeyPress::Up,
            KeyPress::Down,
            KeyPress::Enter,
        ],
        ("a", ""),
    );
}

#[test]
fn up_key() {
    assert_history(&[], &[KeyPress::Up, KeyPress::Enter], ("", ""));
    assert_history(&["line1"], &[KeyPress::Up, KeyPress::Enter], ("line1", ""));
    assert_history(
        &["line1", "line2"],
        &[KeyPress::Up, KeyPress::Up, KeyPress::Enter],
        ("line1", ""),
    );
}

#[test]
fn ctrl_r() {
    assert_history(
        &[],
        &[KeyPress::Ctrl('R'), KeyPress::Char('o'), KeyPress::Enter],
        ("o", ""),
    );
    assert_history(
        &["rustc", "cargo"],
        &[KeyPress::Ctrl('R'), KeyPress::Char('o'), KeyPress::Enter],
        ("cargo", ""),
    );
    assert_history(
        &["rustc", "cargo"],
        &[KeyPress::Ctrl('R'), KeyPress::Char('u'), KeyPress::Enter],
        ("rus", "tc"),
    );
    assert_history(
        &["rustc", "cargo"],
        &[
            KeyPress::Ctrl('R'),
            KeyPress::Char('r'),
            KeyPress::Char('u'),
            KeyPress::Enter,
        ],
        ("rustc", ""),
    );
    assert_history(
        &["rustc", "cargo"],
        &[
            KeyPress::Ctrl('R'),
            KeyPress::Char('r'),
            KeyPress::Char('z'), // no match
            KeyPress::Enter,
        ],
        ("cargo", ""),
    );
    assert_history(
        &["rustc", "cargo"],
        &[
            KeyPress::Char('a'),
            KeyPress::Ctrl('R'),
            KeyPress::Char('r'),
            KeyPress::Ctrl('G'), // abort (FIXME: doesn't work with vi mode)
            KeyPress::Enter,
        ],
        ("a", ""),
    );
}

#[test]
fn meta_lt() {
    assert_history(&[""], &[KeyPress::Meta('<'), KeyPress::Enter], ("", ""));
    assert_history(
        &["rustc", "cargo"],
        &[KeyPress::Meta('<'), KeyPress::Enter],
        ("rustc", ""),
    );
}

#[test]
fn meta_gt() {
    assert_history(&[""], &[KeyPress::Meta('>'), KeyPress::Enter], ("", ""));
    assert_history(
        &["rustc", "cargo"],
        &[KeyPress::Meta('<'), KeyPress::Meta('>'), KeyPress::Enter],
        ("", ""),
    );
    assert_history(
        &["rustc", "cargo"],
        &[
            KeyPress::Char('a'),
            KeyPress::Meta('<'),
            KeyPress::Meta('>'),
            KeyPress::Enter,
        ],
        ("a", ""),
    );
}

extern crate chobitlibs;

use std::prelude::rust_2021::*;

use chobitlibs::chobit_playbook::*;

#[test]
fn direction_test_1() {
    let src =
r#"@hello world\
こんにちは世界\
foo bar"#;

    let book = ChobitPlaybook::parse(src);
    let mut iter = book.iter();

    match iter.next().unwrap() {
        Block::Direction(direction) => {
            let mut iter = direction.iter();

            assert_eq!(iter.next().unwrap(), "hello world");
            assert_eq!(iter.next().unwrap(), "こんにちは世界");
            assert_eq!(iter.next().unwrap(), "foo bar");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Direction");}
    }

    assert!(iter.next().is_none());
}

#[test]
fn direction_test_2() {
    let src =
r#"@hello world\
\
こんにちは世界
@foo\
bar

@alice bob\
carol"#;

    let book = ChobitPlaybook::parse(src);
    let mut iter = book.iter();

    match iter.next().unwrap() {
        Block::Direction(direction) => {
            let mut iter = direction.iter();

            assert_eq!(iter.next().unwrap(), "hello world");
            assert_eq!(iter.next().unwrap(), "");
            assert_eq!(iter.next().unwrap(), "こんにちは世界");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Direction");}
    }

    match iter.next().unwrap() {
        Block::Direction(direction) => {
            let mut iter = direction.iter();

            assert_eq!(iter.next().unwrap(), "foo");
            assert_eq!(iter.next().unwrap(), "bar");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Direction");}
    }

    match iter.next().unwrap() {
        Block::Direction(direction) => {
            let mut iter = direction.iter();

            assert_eq!(iter.next().unwrap(), "alice bob");
            assert_eq!(iter.next().unwrap(), "carol");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Direction");}
    }

    assert!(iter.next().is_none());
}

#[test]
fn text_test_1() {
    let src =
r#"hello world
こんにちは世界
foo bar"#;

    let book = ChobitPlaybook::parse(src);
    let mut iter = book.iter();

    match iter.next().unwrap() {
        Block::Text(text) => {
            let mut iter = text.iter();

            assert_eq!(iter.next().unwrap(), "hello world");
            assert_eq!(iter.next().unwrap(), "こんにちは世界");
            assert_eq!(iter.next().unwrap(), "foo bar");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Text");}
    }

    assert!(iter.next().is_none());
}

#[test]
fn text_test_2() {
    let src =
r#"hello world
\
こんにちは世界

foo
bar

alice bob
carol"#;

    let book = ChobitPlaybook::parse(src);
    let mut iter = book.iter();

    match iter.next().unwrap() {
        Block::Text(text) => {
            let mut iter = text.iter();

            assert_eq!(iter.next().unwrap(), "hello world");
            assert_eq!(iter.next().unwrap(), "");
            assert_eq!(iter.next().unwrap(), "こんにちは世界");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Text");}
    }

    match iter.next().unwrap() {
        Block::Text(text) => {
            let mut iter = text.iter();

            assert_eq!(iter.next().unwrap(), "foo");
            assert_eq!(iter.next().unwrap(), "bar");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Text");}
    }

    match iter.next().unwrap() {
        Block::Text(text) => {
            let mut iter = text.iter();

            assert_eq!(iter.next().unwrap(), "alice bob");
            assert_eq!(iter.next().unwrap(), "carol");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Text");}
    }

    assert!(iter.next().is_none());
}

#[test]
fn playbook_test_1() {
    let src =
r#"
@alice --feeling=smile
Hello!
My name is Alice.

# Comment!

@bob --feeling=crying
\@H...hello...
\My name is... Bo... b...

@alice --feeling=worry\
--symbol=question-mark
What's wrong?



@bob --feeling=crying
@effect flash
I... am... a......
\
\
PEN!
"#;

    let book = ChobitPlaybook::parse(src);
    let mut iter = book.iter();

    match iter.next().unwrap() {
        Block::Direction(direction) => {
            let mut iter = direction.iter();

            assert_eq!(iter.next().unwrap(), "alice --feeling=smile");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Direction");}
    }

    match iter.next().unwrap() {
        Block::Text(text) => {
            let mut iter = text.iter();

            assert_eq!(iter.next().unwrap(), "Hello!");
            assert_eq!(iter.next().unwrap(), "My name is Alice.");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Text");}
    }

    match iter.next().unwrap() {
        Block::Direction(direction) => {
            let mut iter = direction.iter();

            assert_eq!(iter.next().unwrap(), "bob --feeling=crying");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Direction");}
    }

    match iter.next().unwrap() {
        Block::Text(text) => {
            let mut iter = text.iter();

            assert_eq!(iter.next().unwrap(), "@H...hello...");
            assert_eq!(iter.next().unwrap(), "My name is... Bo... b...");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Text");}
    }

    match iter.next().unwrap() {
        Block::Direction(direction) => {
            let mut iter = direction.iter();

            assert_eq!(iter.next().unwrap(), "alice --feeling=worry");
            assert_eq!(iter.next().unwrap(), "--symbol=question-mark");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Direction");}
    }

    match iter.next().unwrap() {
        Block::Text(text) => {
            let mut iter = text.iter();

            assert_eq!(iter.next().unwrap(), "What's wrong?");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Text");}
    }

    match iter.next().unwrap() {
        Block::Direction(direction) => {
            let mut iter = direction.iter();

            assert_eq!(iter.next().unwrap(), "bob --feeling=crying");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Direction");}
    }

    match iter.next().unwrap() {
        Block::Direction(direction) => {
            let mut iter = direction.iter();

            assert_eq!(iter.next().unwrap(), "effect flash");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Direction");}
    }

    match iter.next().unwrap() {
        Block::Text(text) => {
            let mut iter = text.iter();

            assert_eq!(iter.next().unwrap(), "I... am... a......");
            assert_eq!(iter.next().unwrap(), "");
            assert_eq!(iter.next().unwrap(), "");
            assert_eq!(iter.next().unwrap(), "PEN!");

            assert!(iter.next().is_none());
        },

        _ => {panic!("Not Text");}
    }

    assert!(iter.next().is_none());
}

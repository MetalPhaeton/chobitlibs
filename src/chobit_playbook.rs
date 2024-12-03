// Copyright (C) 2023 Hironori Ishibashi
//
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See http://www.wtfpl.net/ for more details.

//! Playbook parser for novel games.
//!
//! # Syntax
//!
//! - If the first letter of a line is `@`, that line is [Block::Direction].
//!     - ...and if the last letter of a line is `\`, it continues to next line.
//! - If the first letter of a line is `#`, that line is comment and treated as an empty line.
//! - If the first letter of a line is `\`, that line is [Block::Text].
//! - If the first letter of a line is neither `@` nor `\`, that line is [Block::Text].
//!
//! - [Block::Text] is continued until appearing an empty line or a line began with `@`.
//!
//! # Example
//!
//! ## Playbook
//!
//! ```text
//! @This is a Direction.
//!
//! # This is a comment.
//!
//! @This\
//! is\
//! a\
//! Direction.
//!
//! This is a Text.
//!
//! \This is a Text.
//!
//! \@This is a Text.
//!
//! This
//! is
//! a
//! Text.
//!
//! This is a Text.
//! @This is a Direction.
//! This is a Text.
//! ```
//!
//! ## Code
//!
//! ```ignore
//! use chobitlibs::chobit_playbook::{
//!     ChobitPlaybook,
//!     Block
//! };
//!
//! let src = r#"
//! @alice
//! Hello!
//! My name is Alice!
//!
//! @bob
//! My name is Bob!
//! \
//! I like chess!
//!
//! @alice\
//! smile
//! \I like it too!
//!"#;
//!
//! let book = ChobitPlaybook::parse(src);
//!
//! let mut iter = book.iter();
//!
//! match iter.next().unwrap() {
//!     Block::Direction(direction) => {
//!         let mut iter = direction.iter();
//!
//!         assert_eq!(iter.next().unwrap(), "alice");
//!
//!         assert!(iter.next().is_none());
//!     },
//!
//!     _ => {panic!("Not Direction");}
//! }
//!
//! match iter.next().unwrap() {
//!     Block::Text(text) => {
//!         let mut iter = text.iter();
//!
//!         assert_eq!(iter.next().unwrap(), "Hello!");
//!         assert_eq!(iter.next().unwrap(), "My name is Alice!");
//!
//!         assert!(iter.next().is_none());
//!     },
//!
//!     _ => {panic!("Not Text");}
//! }
//!
//! match iter.next().unwrap() {
//!     Block::Direction(direction) => {
//!         let mut iter = direction.iter();
//!
//!         assert_eq!(iter.next().unwrap(), "bob");
//!
//!         assert!(iter.next().is_none());
//!     },
//!
//!     _ => {panic!("Not Direction");}
//! }
//!
//! match iter.next().unwrap() {
//!     Block::Text(text) => {
//!         let mut iter = text.iter();
//!
//!         assert_eq!(iter.next().unwrap(), "My name is Bob!");
//!         assert_eq!(iter.next().unwrap(), "");
//!         assert_eq!(iter.next().unwrap(), "I like chess!");
//!
//!         assert!(iter.next().is_none());
//!     },
//!
//!     _ => {panic!("Not Text");}
//! }
//!
//! match iter.next().unwrap() {
//!     Block::Direction(direction) => {
//!         let mut iter = direction.iter();
//!
//!         assert_eq!(iter.next().unwrap(), "alice");
//!         assert_eq!(iter.next().unwrap(), "smile");
//!
//!         assert!(iter.next().is_none());
//!     },
//!
//!     _ => {panic!("Not Direction");}
//! }
//!
//! match iter.next().unwrap() {
//!     Block::Text(text) => {
//!         let mut iter = text.iter();
//!
//!         assert_eq!(iter.next().unwrap(), "I like it too!");
//!
//!         assert!(iter.next().is_none());
//!     },
//!
//!     _ => {panic!("Not Text");}
//! }
//!
//! assert!(iter.next().is_none());
//! ```

#![allow(dead_code)]

use alloc::{string::String, vec::Vec, vec};
use core::{slice::IterMut, iter::Iterator, ops::Deref};

enum Token {
    Empty,
    Comment,
    DirectionFirstLine(String),
    DirectionNextLine(String),
    TextLine(String)
}

type TokenList = Vec<Option<Token>>;

fn tokenize(src: &str) -> TokenList {
    let mut in_direction: bool = false;

    src.split('\n').map(|line| {
        if in_direction {
            let mut line = String::from(line);

            match line.as_bytes().last() {
                Some(last) => {
                    if *last == b'\\' {
                        let _ = line.pop();
                    } else {
                        in_direction = false;
                    }

                    Some(Token::DirectionNextLine(line))
                },

                None => {
                    in_direction = false;

                    Some(Token::DirectionNextLine(line))
                }
            }
        } else {
            match line.as_bytes().get(0) {
                Some(letter) => match *letter {
                    b'@' => {
                        let mut line = String::from(&line[1..]);

                        match line.as_bytes().last() {
                            Some(last) => {
                                if *last == b'\\' {
                                    in_direction = true;

                                    let _ = line.pop();
                                }

                                Some(Token::DirectionFirstLine(line))
                            },

                            None => Some(Token::DirectionFirstLine(line))
                        }
                    },

                    b'\\' => {
                        Some(Token::TextLine(String::from(&line[1..])))
                    },

                    b'#' => {
                        Some(Token::Comment)
                    },

                    _ => if line.is_empty() {
                        Some(Token::Empty)
                    } else {
                        Some(Token::TextLine(String::from(line)))
                    }
                },

                None => Some(Token::Empty)
            }
        }
    }).collect::<TokenList>()
}

/// Block of ChoibtPlaybook.
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// Direction lines.
    Direction(Vec<String>),

    /// Text lines.
    Text(Vec<String>)
}

/// Root of abstract syntax tree of ChobitPlaybook.
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitPlaybook {
    body: Vec<Block>
}

impl ChobitPlaybook {
    /// Parses from source text to `chobit_playbook`.
    ///
    /// - _Return_ : ChobitPlaybook.
    #[inline]
    pub fn parse(src: &str) -> ChobitPlaybook {
        let mut token_list = tokenize(src);
        let iter = token_list.iter_mut();

        ChobitPlaybook {
            body: Self::parse_core(iter)
        }
    }

    #[inline]
    fn parse_core(iter: IterMut<Option<Token>>) -> Vec<Block> {
        let mut ret = Vec::<Block>::new();

        Self::parse_next(&mut ret, iter);

        ret
    }

    fn parse_next(ret: &mut Vec<Block>, mut iter: IterMut<Option<Token>>) {
        if let Some(token_opt) = iter.next() {
            if let Some(token) = token_opt.take() {
                match token {
                    Token::Empty | Token::Comment => {
                        Self::parse_next(ret, iter);
                    },

                    Token::DirectionFirstLine(line) => {
                        Self::parse_direction(ret, line, iter);
                    },

                    Token::DirectionNextLine(..) => {
                        unreachable!("Error at chobit_playbook::ChobitPlaybook::parse_next()");
                    },

                    Token::TextLine(line) => {
                        Self::parse_text(ret, line, iter);
                    }
                }
            }
        }
    }

    fn parse_direction(
        ret: &mut Vec<Block>,
        line: String,
        mut iter: IterMut<Option<Token>>
    ) {
        let mut block = vec![line];

        for token_opt in &mut iter {
            if let Some(token) = token_opt.take() {
                match token {
                    Token::Empty | Token::Comment => {
                        ret.push(Block::Direction(block));

                        Self::parse_next(ret, iter);

                        return;
                    },

                    Token::DirectionFirstLine(line) => {
                        ret.push(Block::Direction(block));

                        Self::parse_direction(ret, line, iter);

                        return;
                    },

                    Token::DirectionNextLine(line) => {
                        block.push(line);
                    },

                    Token::TextLine(line) => {
                        ret.push(Block::Direction(block));

                        Self::parse_text(ret, line, iter);

                        return;
                    }
                }
            }
        }

        ret.push(Block::Direction(block));
    }

    fn parse_text(
        ret: &mut Vec<Block>,
        line: String,
        mut iter: IterMut<Option<Token>>
    ) {
        let mut block = vec![line];

        for token_opt in &mut iter {
            if let Some(token) = token_opt.take() {
                match token {
                    Token::Empty | Token::Comment => {
                        ret.push(Block::Text(block));

                        Self::parse_next(ret, iter);

                        return;
                    },

                    Token::DirectionFirstLine(line) => {
                        ret.push(Block::Text(block));

                        Self::parse_direction(ret, line, iter);

                        return;
                    },

                    Token::DirectionNextLine(..) => {
                        unreachable!("Error at chobit_playbook::ChobitPlaybook::parse_text()");
                    },

                    Token::TextLine(line) => {
                        block.push(line);
                    }
                }
            }
        }

        ret.push(Block::Text(block));
    }
}

impl Deref for ChobitPlaybook {
    type Target = [Block];

    #[inline]
    fn deref(&self) -> &[Block] {
        self.body.as_slice()
    }
}

pub struct Lex<'a> {
    src: &'a str,
}

impl<'a> Lex<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    fn take(&mut self, ptr: usize) -> &'a str {
        let (left, right) = self.src.split_at(ptr);
        self.src = right;
        left
    }

    fn skip_whitespace(&mut self) {
        let mut chars = self.src.chars();
        let mut ptr = 0;
        while let Some(ch @ (' ' | '\t' | '\r' | '\n')) = chars.next() {
            ptr += ch.len_utf8();
        }

        self.take(ptr);
    }

    fn tok(&mut self) -> Option<(Class<'a>, &'a str)> {
        fn is_digit(ch: char) -> bool {
            matches!(ch, '0'..='9')
        }

        fn is_alpha(ch: char) -> bool {
            matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
        }

        self.skip_whitespace();
        let mut chars = self.src.chars().peekable();
        let ch = chars.next()?;
        let mut ptr = ch.len_utf8();

        match ch {
            '#' => {
                for ch in chars {
                    if ch == '\n' {
                        break;
                    }
                    ptr += ch.len_utf8();
                }

                let st = self.take(ptr);
                Some((Class::Comment(&st[1..]), st))
            }
            '-' => Some((Class::Minus, self.take(ptr))),
            '\'' => {
                let mut escape = false;

                // Escape all chars, more detailed string parsing at the level above
                for ch in chars {
                    match ch {
                        '\\' => escape = true,
                        '\'' if !escape => {
                            ptr += ch.len_utf8();
                            break;
                        }
                        _ if escape => escape = false,
                        _ => {}
                    }

                    ptr += ch.len_utf8();
                }

                let st = self.take(ptr);
                Some((Class::Str(&st[1..st.len() - 1]), st))
            }
            _ if is_digit(ch) => {
                let mut radix = 10;
                if ch == '0' {
                    'radix: {
                        radix = match chars.peek() {
                            Some('x') => 16,
                            Some('b') => 2,
                            _ => break 'radix,
                        };

                        chars.next();
                        ptr += 1;
                    }
                }

                let mut dot = false;
                for ch in chars {
                    if ch == '.' && !dot {
                        dot = true;
                    } else if !is_alpha(ch) && !is_digit(ch) {
                        break;
                    }

                    ptr += ch.len_utf8();
                }

                let st = self.take(ptr);
                let cl = if dot {
                    st.parse().ok().map(Class::Flt)
                } else {
                    let n = if radix == 10 { st } else { &st[2..] };
                    u32::from_str_radix(n, radix).ok().map(Class::Num)
                };

                Some((cl.unwrap_or(Class::Undefined), st))
            }
            _ if is_alpha(ch) => {
                for ch in chars {
                    if !is_alpha(ch) && !is_digit(ch) {
                        break;
                    }

                    ptr += ch.len_utf8();
                }

                let st = self.take(ptr);
                Some((Class::Name(st), st))
            }
            _ => Some((Class::Undefined, self.take(ptr))),
        }
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = Tok<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tok().map(|(class, span)| Tok { class, span })
    }
}

#[derive(Debug, PartialEq)]
pub struct Tok<'a> {
    pub class: Class<'a>,
    pub span: &'a str,
}

impl<'a> Tok<'a> {
    /// Returns token's byte length.
    pub fn len(&self) -> usize {
        self.span.len()
    }

    /// Returns token's offset in src.
    pub fn offset_in(&self, src: &'a str) -> usize {
        let src_start = src.as_ptr() as usize;
        let begin = self.span.as_ptr() as usize;
        begin - src_start
    }
}

#[derive(Debug, PartialEq)]
pub enum Class<'a> {
    Num(u32),
    Flt(f32),
    Name(&'a str),
    Str(&'a str),
    Minus,
    Comment(&'a str),
    Undefined,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let src = r#"
            'hi' '\'' ''
            '#str'
            foo 12 0x1F
            0b101 1.1 &
            -5 #hi
            #lol
        "#;

        let lx: Vec<_> = Lex::new(src).collect();
        assert_eq!(
            lx,
            [
                Tok {
                    class: Class::Str("hi"),
                    span: "'hi'"
                },
                Tok {
                    class: Class::Str("\\'"),
                    span: "'\\''"
                },
                Tok {
                    class: Class::Str(""),
                    span: "''"
                },
                Tok {
                    class: Class::Str("#str"),
                    span: "\'#str\'",
                },
                Tok {
                    class: Class::Name("foo"),
                    span: "foo"
                },
                Tok {
                    class: Class::Num(12),
                    span: "12"
                },
                Tok {
                    class: Class::Num(0x1F),
                    span: "0x1F"
                },
                Tok {
                    class: Class::Num(0b101),
                    span: "0b101"
                },
                Tok {
                    class: Class::Flt(1.1),
                    span: "1.1"
                },
                Tok {
                    class: Class::Undefined,
                    span: "&"
                },
                Tok {
                    class: Class::Minus,
                    span: "-"
                },
                Tok {
                    class: Class::Num(5),
                    span: "5"
                },
                Tok {
                    class: Class::Comment("hi"),
                    span: "#hi"
                },
                Tok {
                    class: Class::Comment("lol"),
                    span: "#lol"
                },
            ]
        );
    }
}

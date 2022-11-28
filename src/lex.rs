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
            matches!(ch, 'a'..='z' | 'A'..='Z')
        }

        self.skip_whitespace();
        let mut chars = self.src.chars().peekable();
        let ch = chars.next()?;
        let mut ptr = ch.len_utf8();

        match ch {
            '#' => {
                for ch in &mut chars {
                    if ch == '\n' {
                        break;
                    }
                    ptr += ch.len_utf8();
                }

                let st = self.take(ptr);
                Some((Class::Comment(&st[1..]), st))
            }
            ch if is_digit(ch) => {
                let mut radix = 10;
                let mut dot = false;

                if ch == '0' {
                    'skip: {
                        let ch;
                        (ch, radix) = match chars.peek() {
                            Some(&ch @ 'x') => (ch, 16),
                            Some(&ch @ 'b') => (ch, 2),
                            _ => break 'skip,
                        };

                        chars.next();
                        ptr += ch.len_utf8();
                    }
                }

                for ch in &mut chars {
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
            ch if is_alpha(ch) => {
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

#[derive(Debug, PartialEq)]
pub enum Class<'a> {
    Num(u32),
    Flt(f32),
    Name(&'a str),
    Comment(&'a str),
    Undefined,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let src = r#"
            foo 12 0x1F
            0b101 1.1 &
            5 #hi
            #lol
        "#;

        let lx: Vec<_> = Lex::new(src).collect();
        assert_eq!(
            lx,
            [
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

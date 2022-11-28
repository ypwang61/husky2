use husky_print_utils::p;

use crate::*;

impl<'a> TokenIter<'a> {
    #[allow(clippy::type_complexity)]
    fn next_string(
        &mut self,
        delim: char,
        new_ch: &mut dyn FnMut(
            &mut TokenIter<'_>,
            &mut MaybeString,
            bool,
            usize,
            char,
        ) -> TomlTokenResult<()>,
    ) -> TomlTokenResult<TomlTokenVariant> {
        let mut multiline = false;
        if self.try_eat_char(/* second */ delim) {
            if self.try_eat_char(/* third */ delim) {
                multiline = true;
            } else {
                return Ok(TomlTokenVariant::StringLiteral {
                    val: Default::default(),
                    multiline: false,
                });
            }
        }
        let mut val = MaybeString::NotEscaped(self.current());
        let mut n = 0;
        'outer: loop {
            n += 1;
            match self.next_char_with_offset() {
                Some((i, '\n')) => {
                    if multiline {
                        if self.input.as_bytes()[i] == b'\r' {
                            val.to_owned(&self.input[..i]);
                        }
                        if n == 1 {
                            val = MaybeString::NotEscaped(self.current());
                        } else {
                            val.push('\n');
                        }
                        continue;
                    } else {
                        return Err(TomlTokenError::NewlineInString(i));
                    }
                }
                Some((mut i, ch)) if ch == delim => {
                    if multiline {
                        if !self.try_eat_char(delim) {
                            val.push(delim);
                            continue 'outer;
                        }
                        if !self.try_eat_char(delim) {
                            val.push(delim);
                            val.push(delim);
                            continue 'outer;
                        }
                        if self.try_eat_char(delim) {
                            val.push(delim);
                            i += 1;
                        }
                        if self.try_eat_char(delim) {
                            val.push(delim);
                            i += 1;
                        }
                    }
                    return Ok(TomlTokenVariant::StringLiteral {
                        val: val.into_cow(&self.input[..i]),
                        multiline,
                    });
                }
                Some((i, c)) => new_ch(self, &mut val, multiline, i, c)?,
                None => return Err(TomlTokenError::UnterminatedString),
            }
        }
    }

    pub(crate) fn next_literal_string(&mut self) -> TomlTokenResult<TomlTokenVariant> {
        self.next_string('\'', &mut |_me, val, _multi, i, ch| {
            if ch == '\u{09}' || (('\u{20}'..='\u{10ffff}').contains(&ch) && ch != '\u{7f}') {
                val.push(ch);
                Ok(())
            } else {
                Err(TomlTokenError::InvalidCharInString(i, ch))
            }
        })
    }

    pub(crate) fn next_basic_string(&mut self) -> TomlTokenResult<TomlTokenVariant> {
        self.next_string('"', &mut |this, val, multi, i, ch| match ch {
            '\\' => {
                val.to_owned(&this.input[..i]);
                match this.next_char_with_offset() {
                    Some((_, '"')) => val.push('"'),
                    Some((_, '\\')) => val.push('\\'),
                    Some((_, 'b')) => val.push('\u{8}'),
                    Some((_, 'f')) => val.push('\u{c}'),
                    Some((_, 'n')) => val.push('\n'),
                    Some((_, 'r')) => val.push('\r'),
                    Some((_, 't')) => val.push('\t'),
                    Some((i, c @ 'u')) | Some((i, c @ 'U')) => {
                        let len = if c == 'u' { 4 } else { 8 };
                        val.push(this.next_hex(i, len)?);
                    }
                    Some((i, c @ ' ')) | Some((i, c @ '\t')) | Some((i, c @ '\n')) if multi => {
                        if c != '\n' {
                            while let Some(ch) = this.peek_char() {
                                match ch {
                                    ' ' | '\t' => {
                                        this.next_char();
                                        continue;
                                    }
                                    '\n' => {
                                        this.next_char();
                                        break;
                                    }
                                    _ => return Err(TomlTokenError::InvalidEscape(i, c)),
                                }
                            }
                        }
                        while let Some(ch) = this.peek_char() {
                            match ch {
                                ' ' | '\t' | '\n' => {
                                    this.next_char();
                                }
                                _ => break,
                            }
                        }
                    }
                    Some((i, c)) => return Err(TomlTokenError::InvalidEscape(i, c)),
                    None => return Err(TomlTokenError::UnterminatedString),
                }
                Ok(())
            }
            ch if ch == '\u{09}' || (('\u{20}'..='\u{10ffff}').contains(&ch) && ch != '\u{7f}') => {
                val.push(ch);
                Ok(())
            }
            _ => Err(TomlTokenError::InvalidCharInString(i, ch)),
        })
    }
}

#[derive(Debug)]
enum MaybeString {
    NotEscaped(usize),
    Owned(std::string::String),
}

impl MaybeString {
    fn push(&mut self, ch: char) {
        match *self {
            MaybeString::NotEscaped(..) => {}
            MaybeString::Owned(ref mut s) => s.push(ch),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_owned(&mut self, input: &str) {
        match *self {
            MaybeString::NotEscaped(start) => {
                *self = MaybeString::Owned(input[start..].to_owned());
            }
            MaybeString::Owned(..) => {}
        }
    }

    fn into_cow(self, input: &str) -> StringValue {
        match self {
            MaybeString::NotEscaped(start) => Arc::new(input[start..].to_owned()),
            MaybeString::Owned(s) => Arc::new(s),
        }
    }
}
//! This module implements a zero-copy recursive descent [Parser] for parsing the Quake Log.

use std::{iter::Peekable, str::Chars};

use num_traits::FromPrimitive;

use crate::events::{ClientUserInfoChanged, Event, EventKind, Kill};
pub(crate) struct Parser<'a> {
    peekable: Peekable<Chars<'a>>,
    input: &'a str,
    position: usize,
}

// Base functions for parsing.
impl<'a> Parser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            peekable: input.chars().peekable(),
            input,
            position: 0,
        }
    }

    /// Gets the next character, and advances the peekable and the position
    pub(crate) fn next(&mut self) -> Option<char> {
        let char = self.peekable.next()?;
        self.position += char.len_utf8();
        Some(char)
    }

    /// Gets the next character
    #[inline(always)]
    pub(crate) fn peek(&mut self) -> Option<&char> {
        self.peekable.peek()
    }

    /// Skip characters if they match a predicate
    pub(crate) fn skip(&mut self, predicate: fn(&char) -> bool) {
        while self.peek().map(predicate).unwrap_or_default() {
            self.next();
        }
    }

    pub(crate) fn expect(&mut self, expect: char) -> Option<char> {
        let char = *self.peek()?;
        if char == expect {
            self.next();
            Some(char)
        } else {
            None
        }
    }

    /// Gets the string that the predicate parses.
    pub(crate) fn splice<T>(
        &mut self,
        predicate: &dyn Fn(&mut Self) -> Option<T>,
    ) -> Option<&'a str> {
        let start = self.position;
        predicate(self)?;
        let pos = self.position;
        Some(&self.input[start..pos])
    }

    pub(crate) fn accumulate(&mut self, predicate: fn(&char) -> bool) -> Option<&'a str> {
        self.splice(&|this| {
            this.skip(predicate);
            Some(())
        })
    }
}

impl<'a> Parser<'a> {
    pub fn number(&mut self) -> Option<u32> {
        let str = self.accumulate(char::is_ascii_digit)?;
        str.parse().ok()
    }

    pub fn parse_hour(&mut self) -> Option<(u32, u32)> {
        let hour = self.number()?;
        self.expect(':')?;
        let minutes = self.number()?;
        Some((hour, minutes))
    }

    pub fn skip_whitespace(&mut self) {
        self.skip(|x| char::is_whitespace(*x));
    }

    pub fn parse_event(&mut self) -> Option<&str> {
        self.accumulate(|x| char::is_alphabetic(*x))
    }

    pub fn parse_kill(&mut self) -> Option<Kill> {
        let killer = self.number()?;
        self.expect(' ')?;
        let victim = self.number()?;
        self.expect(' ')?;
        let item = self.number()?;
        self.expect(':')?;
        Some(Kill {
            victim,
            killer,
            mean: FromPrimitive::from_u32(item)?,
        })
    }

    pub fn parse_kill_event(&mut self) -> Option<EventKind<'a>> {
        let kill = self.parse_kill()?;
        Some(EventKind::Kill(kill))
    }

    pub fn parse_client_user_changed(&mut self) -> Option<EventKind<'a>> {
        self.skip_whitespace();
        let id = self.number()?;
        self.skip_whitespace();
        self.expect('n');
        self.expect('\\');
        let nickname = self.accumulate(|x| *x != '\\')?;
        Some(EventKind::ClientUserInfoChanged(ClientUserInfoChanged {
            id,
            nickname,
        }))
    }

    pub fn parse(&mut self) -> Option<Event<'a>> {
        self.skip_whitespace();
        let date = self.parse_hour()?;
        self.skip_whitespace();
        let event = self.parse_event()?.to_string();
        self.expect(':')?;

        self.skip_whitespace();

        let kind = match event.as_str() {
            "Kill" => self.parse_kill_event()?,
            "ClientUserinfoChanged" => self.parse_client_user_changed()?,
            "InitGame" => EventKind::InitGame,
            "ShutdownGame" => EventKind::ShutdownGame,
            _ => EventKind::Irrelevant,
        };

        Some(Event { date, kind })
    }
}

/// Parses a string to an event
pub fn parse(input: &str) -> Option<Event<'_>> {
    let mut parser = Parser::new(input);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use crate::events::{ClientUserInfoChanged, EventKind, Kill, MeansOfDeath};

    use super::parse;

    #[test]
    pub fn test_kill() {
        let kill = "  4:12 Kill: 5 3 7: Dono da Bola killed Isgalamido by MOD_ROCKET_SPLASH";
        let parsed = parse(kill);

        assert!(parsed.is_some());

        let parsed = parsed.unwrap();

        assert!(matches!(
            parsed.kind,
            EventKind::Kill(Kill {
                victim: 3,
                killer: 5,
                mean: MeansOfDeath::ModRocketSplash
            })
        ))
    }

    #[test]
    pub fn test_change_info() {
        let change_info = "  5:55 ClientUserinfoChanged: 8 n\\Chessus\\t\\0\\model\\visor/blue\\hmodel\\visor/blue\\g_redteam\\g_blueteam";
        let parsed = parse(change_info);

        assert!(parsed.is_some());

        let parsed = parsed.unwrap();

        assert!(matches!(
            parsed.kind,
            EventKind::ClientUserInfoChanged(ClientUserInfoChanged {
                id: 8,
                nickname: "Chessus"
            })
        ))
    }
}

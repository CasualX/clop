use std::str;
use std::ops::Range;

fn is_whitespace(chr: u8) -> bool {
	match chr {
		0...31 => (1 << chr) & 0x2600 != 0,
		32 => true,
		33...255 => false,
		_ => unreachable!()
	}
}

/// Tokenizes a command line string producing string slices.
pub struct Tokens<'a> {
	s: &'a str,
}
impl<'a> Tokens<'a> {
	/// Returns the remainder of the command string.
	pub fn tail_str(&self) -> &'a str {
		self.s
	}
}
impl<'a> From<&'a str> for Tokens<'a> {
	fn from(s: &'a str) -> Tokens<'a> {
		Tokens { s }
	}
}
impl<'a> Iterator for Tokens<'a> {
	type Item = &'a str;
	fn next(&mut self) -> Option<&'a str> {
		let s = self.s.as_bytes();
		let mut i = 0;
		while s.get(i).cloned().map(is_whitespace).unwrap_or(false) {
			i += 1;
		}
		if i >= s.len() {
			self.s = unsafe { str::from_utf8_unchecked(&s[i..]) };
			return None;
		}
		let start = i;
		while let Some(&chr) = s.get(i) {
			if is_whitespace(chr) {
				break;
			}
			i += 1;
			if chr == b'"' {
				while let Some(&chr) = s.get(i) {
					i += 1;
					if chr == b'"' {
						break;
					}
				}
			}
		}
		let end = i;
		let s = unsafe { self.s.get_unchecked(start..end) };
		self.s = unsafe { self.s.get_unchecked(end..) };
		Some(s)
	}
}

/// Tokenizes a command line string producing ranges.
pub struct TokenRanges<'a> {
	s: &'a str,
	i: usize,
}
impl<'a> TokenRanges<'a> {
	/// Returns the original command string.
	pub fn as_str(&self) -> &'a str {
		self.s
	}
	/// Returns the remainder of the command string.
	pub fn tail_str(&self) -> &'a str {
		unsafe { self.s.get_unchecked(self.i..) }
	}
}
impl<'a> From<&'a str> for TokenRanges<'a> {
	fn from(s: &'a str) -> TokenRanges<'a> {
		TokenRanges { s, i: 0 }
	}
}
impl<'a> Iterator for TokenRanges<'a> {
	type Item = Range<usize>;
	fn next(&mut self) -> Option<Range<usize>> {
		let s = self.s.as_bytes();
		let mut i = self.i;
		while s.get(i).cloned().map(is_whitespace).unwrap_or(false) {
			i += 1;
		}
		if i >= s.len() {
			self.i = i;
			return None;
		}
		let start = i;
		while let Some(&chr) = s.get(i) {
			if is_whitespace(chr) {
				break;
			}
			i += 1;
			if chr == b'"' {
				while let Some(&chr) = s.get(i) {
					i += 1;
					if chr == b'"' {
						break;
					}
				}
			}
		}
		let end = i;
		self.i = end;
		Some(start..end)
	}
}

#[test]
fn test_simple() {
	let mut tokens = Tokens::from("hello world  ");
	let mut tokens_r = TokenRanges::from("  hello world");

	assert_eq!(Some("hello"), tokens.next());
	assert_eq!(Some(2..7), tokens_r.next());

	assert_eq!(Some("world"), tokens.next());
	assert_eq!(Some(8..13), tokens_r.next());

	assert_eq!(None, tokens.next());
	assert_eq!(None, tokens_r.next());
}

#[test]
fn test_quotes() {
	let mut tokens = Tokens::from("  \"cmdopt.exe\"	some\" arg \" +1");
	let mut tokens_r = TokenRanges::from("\"cmdopt.exe\"	some\" arg \" +1  ");

	assert_eq!(Some("\"cmdopt.exe\""), tokens.next());
	assert_eq!(Some(0..12), tokens_r.next());

	assert_eq!(Some("some\" arg \""), tokens.next());
	assert_eq!(Some(13..24), tokens_r.next());

	assert_eq!(Some("+1"), tokens.next());
	assert_eq!(Some(25..27), tokens_r.next());

	assert_eq!(None, tokens.next());
	assert_eq!(None, tokens_r.next());
}

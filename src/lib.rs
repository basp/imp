pub struct Lexer<'a> {
	src: &'a [u8],
	pos: usize
}

impl<'a> Lexer<'a> {
	pub fn new(src: &'a [u8]) -> Lexer {
		Lexer { src: src, pos: 0 }
	}

	fn is_ws(ch: u8) -> bool {
		match ch {
			b' '	=> true,
			b'\r'	=> true,
			b'\n'	=> true,
			b'\t'	=> true,
			_		=> false,
		}
	}
}

impl<'a> Lexer<'a> {
	fn eof(&self) -> bool { 
		self.pos >= self.src.len() 
	}

	fn ch(&self) -> Option<u8> {
		match self.eof() {
			true	=> None,
			false	=> Some(self.src[self.pos])
		}
	}

	fn string(&mut self) -> Option<String> {
		let mut tok = Vec::<u8>::new();

		loop {
			let ch = self.ch();
			match ch {
				Some(c) if Lexer::is_ws(c)	=> break,
				Some(b'"')					=> break,
				None						=> break,
				_							=> ()
			}

			tok.push(ch.unwrap());
			self.pos += 1;
		}

		match tok.len() {
			0	=> None,
			_	=> Some(String::from_utf8(tok).unwrap())
		}
	}

	fn quoted_string(&mut self) -> Option<String> {
		let mut tok = Vec::<u8>::new();
		self.pos += 1;

		loop {
			let ch = self.ch();
			match ch {
				Some(b'"')		=> { self.pos += 1; break; },
				None			=> break,
				_				=> ()
			}

			tok.push(ch.unwrap());
			self.pos += 1;
		}

		match tok.len() {
			0	=> None,
			_	=> Some(String::from_utf8(tok).unwrap())
		}
	}

	fn skip_ws(&mut self) -> usize {
		let prev_pos = self.pos;
		while self.valid_ws() {
			self.pos += 1;
		}
		self.pos - prev_pos
	}

	fn valid_ws(&self) -> bool {
		match self.ch() {
			Some(c)		=> Lexer::is_ws(c),
			None		=> false
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = String;

	fn next(&mut self) -> Option<String> {
		self.skip_ws();
		match self.ch() {
			Some(b'"')	=> self.quoted_string(),
			Some(_)		=> self.string(),
			None		=> None
		}
	}
}

#[test]
fn eof() {
	assert!(Lexer::new(b"").eof());
	assert!(!Lexer::new(b"a").eof());
}

#[test]
fn valid_ws() {
	let tests = vec![
		(b" ", 	true),
		(b"\t",	true),
		(b"\n",	true),
		(b"\r",	true),
		(b"a", 	false)
	];

	let mut lexer;
	for (src, expected) in tests {
		lexer = Lexer::new(src);
		assert_eq!(expected, lexer.valid_ws());
	}
}

#[test]
fn tokenize() {
	let lexer = Lexer::new(b"foo bar baz \"a b\" zoz");
	let tokens: Vec<_> = lexer.collect();
	assert_eq!("foo".to_string(), tokens[0]);
	assert_eq!("bar".to_string(), tokens[1]);
	assert_eq!("baz".to_string(), tokens[2]);
	assert_eq!("a b".to_string(), tokens[3]);
	assert_eq!("zoz".to_string(), tokens[4]);
}
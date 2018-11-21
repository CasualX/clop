/*!
Command line option parser.

# Examples

```
let line = "command -o --long-arg value -- target 1 2 3 4 5";

let mut state = clop::State::Command;
let mut opt_name = "";
for s in clop::Tokens::from(line) {
	// Expecting to start with the command
	if let clop::State::Command = state {
		// After the command we're expecting a number of options
		state = clop::State::Options;
		continue;
	}
	if let clop::State::Options = state {
		if s == "-o" {
			println!("Got the -o option");
			continue;
		}
		else if s == "--long-arg" {
			opt_name = s;
			state = clop::State::Value;
			continue;
		}
		else if s == "--" {
			println!("No more options");
			state = clop::State::Fixed(0);
			continue;
		}
		else if s.starts_with("-") {
			println!("Unknown option: {}", s);
			continue;
		}
		// Next expecting a fixed argument, target
		state = clop::State::Fixed(0);
	}
	else if let clop::State::Value = state {
		if opt_name == "--long-arg" {
			println!("Got value for {}: {}", opt_name, s);
			state = clop::State::Options;
			continue;
		}
		else {
			unreachable!()
		}
	}
	if let clop::State::Fixed(_) = state {
		println!("Got the fixed argument target: {}", s);
		state = clop::State::Args;
		continue;
	}
	if let clop::State::Args = state {
		println!("Got a variable argument: {}", s);
		continue;
	}
}
```
 */

mod tokens;

pub use self::tokens::{Tokens, TokenRanges};

/// Generic state machine for parsing command line arguments.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
	/// Expecting a command.
	Command,
	/// Expecting a number of modifiers.
	Modifiers,
	/// Expecting a number of options.
	Options,
	/// Expecting an option value.
	Value,
	/// Expecting fixed position argument.
	Fixed(u32),
	/// Expecting variable arguments.
	Args,
	/// Expecting no more arguments.
	End,
}

extern crate clop;

fn main() {
	// Holds the current parser state
	let mut state = clop::State::Command;

	// When parsing options with values as a separate argument,
	// the value needs to know what option it is associated with.
	let mut opt_name = String::new();

	for token in std::env::args() {
		// Expecting to start with the command
		if let clop::State::Command = state {
			// After the command we're expecting a number of options
			state = clop::State::Options;
			continue;
		}
		else if let clop::State::Options = state {
			if token == "-o" {
				println!("Got the -o option");
				continue;
			}
			else if token == "--long-arg" {
				opt_name = token;
				state = clop::State::Value;
				continue;
			}
			else if token == "--" {
				println!("No more options");
				state = clop::State::Fixed(0);
				continue;
			}
			else if token.starts_with("-") {
				println!("Unknown option: {}", token);
				continue;
			}
			// Next expecting a fixed argument, target
			state = clop::State::Fixed(0);
			// Fallthrough is on purpose!
			// The current iteration `token` holds the fixed argument
		}
		else if let clop::State::Value = state {
			if opt_name == "--long-arg" {
				println!("Got value for {}: {}", opt_name, token);
				state = clop::State::Options;
				continue;
			}
			else {
				unreachable!()
			}
		}
		if let clop::State::Fixed(_) = state {
			println!("Got the fixed argument target: {}", token);
			state = clop::State::Args;
			continue;
		}
		if let clop::State::Args = state {
			println!("Got a variable argument: {}", token);
			continue;
		}
	}
}

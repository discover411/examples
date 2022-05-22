use colored::Colorize;
use rustyline::{ Editor, KeyEvent, KeyCode, Modifiers, Cmd, error::ReadlineError };

use lexing_toy::*;

fn main() {
	// this Editor stuff just makes it much nicer to use this program on the command line.
	// it's not important for understanding the lexer at all.
	let mut rl = Editor::<()>::new();
	rl.bind_sequence(KeyEvent(KeyCode::Tab, Modifiers::NONE),   Cmd::Insert(1, "\t".into()));
	rl.bind_sequence(KeyEvent(KeyCode::Down, Modifiers::SHIFT), Cmd::Insert(1, "\n".into()));

	println!("{}",
		"--------------------------------------------------------------------------".bright_blue());
	println!("{}",
		"Type some code. Shift+Down inserts a newline; Enter submits; Ctrl+C quits.".bright_blue());
	println!("{}",
		"--------------------------------------------------------------------------".bright_blue());

	loop {
		let line = rl.readline(">> ");
		match line {
			// we got a line of text!
			Ok(line) => {
				rl.add_history_entry(line.as_str());
				show_tokens(&line);
			}
			// they hit ctrl+C.
			Err(ReadlineError::Interrupted) => break,
			// they hit ctrl+D.
			Err(ReadlineError::Eof) => break,
			// some other error happened.
			Err(err) => {
				println!("Error: {:?}", err);
				break
			}
		}
	}

	println!("byeeeeee!");
}

// this is the function that actually calls the lexer and displays the tokens/errors.
fn show_tokens(line: &str) {
	// since lex() returns a Result, we have to match and see if it succeeded (Ok) or failed (Err).
	match lex(line) {
		Ok(tokens) => {
			// lexing succeeded, print out the tokens.
			println!("{} ", "Tokens:".green());

			for t in tokens {
				println!("   {:?}", t);
			}
		}
		// lexing failed, print out the error.
		Err(e) => println!("{} {}", "Error:".red(), e),
	}

	println!();
}
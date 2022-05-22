fn main() {
	// all the interesting code is down in these functions. main just calls them.
	int_match(0);
	int_match(1);
	int_match(2);

	str_match("hello");
	str_match("goodbye");

	char_match('x');
	char_match('5');
	char_match(' ');
	char_match('@');

	option_match(None);
	option_match(Some(7));
}

fn int_match(x: i32) {
	// "match" works a bit like switch-case, but it's a lot more powerful and not as weird as Java.
	// each "case" is properly called an "arm". you'll see that in error messages too. this match
	// has 3 arms.
	match x {
		// each arm looks like: pattern => code
		// the pattern is the value you're checking for, and the code is what's run when the
		// value matches that pattern.

		// for short code, you can just write the code with a comma after it.
		// sometimes this can cause weird typechecking issues though, so... (see next arm)
		0 => println!("int_match: it's zero!"),

		// you can also use a curly-brace-block for the code, which lets you put multiple
		// lines or other control structures inside it. this avoids weird typechecking issues too.
		1 => {
			println!("int_match: it's one!");
		}

		// a single underscore for the pattern means "anything else," so this works the same
		// as "default" in a Java switch. Rust also forces you to write this arm last.
		_ => {
			println!("int_match: it's something other than 0 or 1.");
		}
	}
}

// note the argument type for s: &str. &str means "a reference to a string" and is typically
// the type you want to use for string arguments. I'm not gonna get into it.
fn str_match(s: &str) {
	// match works on many types, not just ints. (Java did add string switches a while ago, so
	// this isn't *too* different.)
	match s {
		"hello" => println!("str_match says hello back!"),
		_       => println!("str_match doesn't know what you mean..."),
	}
}

fn char_match(c: char) {
	// when matching on ints and chars, you can also use *ranges* for the arm patterns. this is
	// very relevant for lexing, where you're typically checking for these ranges!
	match c {
		// I'm using the ..= to mean "upper bound inclusive", so this means "a to z inclusive".
		'a' ..= 'z' => println!("char_match: '{}' is a lowercase letter.", c),
		'0' ..= '9' => println!("char_match: '{}' is a digit.", c),

		// this is a more complex pattern composed of multiple patterns. it's read as "space or
		// tab or newline". if c matches any of these, this arm is run.
		' ' | '\t' | '\n' => println!("char_match: '{}' is whitespace.", c),

		// again, the default case.
		_ => println!("char_match: '{}' is something else.", c),
	}
}

// this takes an *optional* i32 (int) argument.
fn option_match(o: Option<i32>) {
	// Options can be one of two values: None (like null) or Some(x), where x is the value that
	// it contains. A match is one way of "unwrapping" the value out of an Option.

	match o {
		None => println!("option_match: you didn't give me anything!"),

		// !!!!!!!!!IMPORTANT!!!!!!!!!
		// something that a lot of people didn't get in the past: this pattern Some(x) actually
		// declares a new variable "x". It is assigned the value that was wrapped up in the Option,
		// and can be used in the arm's code.
		Some(x) => {
			println!("option_match: you gave me the number {}.", x);
		}
	}

	// this way of unpacking an Option works, but it's a bit verbose, so there is another example
	// that shows ways of using Options.
}
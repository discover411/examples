
// Each enum variant is allowed to have data associated with it, like its own
// mini-struct.
#[derive(Debug)]
enum AnimalKind {
	Cat   { pattern: String },
	Dog   { loudness: i32   },
	Camel { num_humps: i32  },
}

// The Animal struct holds common things, as well as the 'kind' field, which holds
// the varying part (the per-animal data).
#[derive(Debug)]
struct Animal {
	weight: f32,
	kind:   AnimalKind,
}

impl Animal {
	// Constructors for each kind of animal.
	fn new_cat(weight: f32, pattern: &str) -> Animal {
		Animal {
			weight,
			kind: AnimalKind::Cat {
				pattern: pattern.into()
			}
		}
	}

	fn new_dog(weight: f32, loudness: i32) -> Animal {
		Animal {
			weight,
			kind: AnimalKind::Dog { loudness }
		}
	}

	fn new_camel(weight: f32, num_humps: i32) -> Animal {
		Animal {
			weight,
			kind: AnimalKind::Camel { num_humps }
		}
	}

	// Methods that work on any kind of animal.
	fn weight(&self) -> f32 {
		self.weight
	}

	fn speak(&self) {
		// this makes the code in the match a little shorter and easier to read.
		use AnimalKind::*;

		match &self.kind {
			// the .. in these patterns says "I don't care, whatever". all we care
			// about here is the "discriminant" - the thing that says what animal it is.
			Cat { .. }   => println!("meow!"),
			Dog { .. }   => println!("woof!"),
			Camel { .. } => println!("ghhghgh!"),
		}
	}

	fn describe(&self) {
		use AnimalKind::*;

		match &self.kind {
			// When you put the field names in the patterns, it is actually *declaring those as
			// variables* which you can then use within the arm's code.
			// Also you can use { brace blocks } for the arm's code, if you have multiple lines.
			Cat { pattern } => println!("this is a {} cat.", pattern),
			Dog { loudness } => {
				// the *loudness here is for Reasons. uhhhhh. I won't get into it.
				if *loudness < 100 {
					println!("this is a quiet dog.");
				} else if *loudness < 1000 {
					println!("this is a pretty loud dog.");
				} else {
					println!("this is a very loud dog.");
				}
			}
			Camel { num_humps } => {
				// look, a match inside a match!

				match num_humps {
					1 => println!("this is a dromedary camel."),
					2 => println!("this is a bactrian camel."),
					_ => println!("this is some exotic kind of camel unknown to science."),
				}
			}
		}
	}

	// A method that only works on cats.
	fn play_with_yarn(&self) {
		if let AnimalKind::Cat { pattern } = &self.kind {
			println!("The {} cat plays with some yarn. Reoowwww", pattern);
		} else {
			// this just crashes.
			panic!("play_with_yarn called on a non-cat");
		}
	}
}

fn main() {
	// Let's make some animals!
	let animals = vec![
		Animal::new_cat(12.0, "Tuxedo"), // who could THIS be?
		Animal::new_dog(43.0, 99999),    // very loud. ow
		Animal::new_camel(1567.0, 2),    // it's a Bactrian
	];

	// {:#?} debug-prints things out with newlines and indentation.
	println!("My animals: {:#?}", animals);

	// Let's sum up all their weights!
	let total_weight: f32 = animals.iter().map(|a| a.weight()).sum();
	println!("All the animals together weigh {} pounds.", total_weight);

	// Let's make them speak!
	println!("Let's see what they have to say: ");
	for a in &animals {
		print!("  ");
		a.speak();
	}

	// Let's describe them!
	println!("What are they like?");
	for a in &animals {
		print!("  ");
		a.describe();
	}

	// And finally, a cat-specific method.
	print!("Time to play: ");
	animals[0].play_with_yarn();

	// Uncommenting this will crash the program!
	// animals[1].play_with_yarn();
}

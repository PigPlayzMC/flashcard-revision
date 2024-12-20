use rand::Rng;
use rusqlite::{params, Connection};
use core::panic;
use std::io;
// Will eventually need to use chrono but not now...

// All flashcards follow this structure
#[derive(Clone)]
pub struct Flashcard {
	primary_key: i32, // Primary key of the flashcard
	question: String, // Question or front text of the card
	answer: String, // Answer or back text of the card
	correct: i32, // Times the user has gotten the question correct
	incorrect: i32, // Times the user has gotten the question incorrect
}

// ## SQLite functions ##
fn add_new_flashcard(conn: &Connection, ques: String, ans: String, subject_name: &str) {
	// This takes inputs and adds it to the correct database, based on the subject.
	let _ = conn.execute(
		format!("INSERT INTO {} (category, question, answer, correct, incorrect) VALUES (?1, ?2, ?3, ?4, ?5);", subject_name).as_str(),
		params![0, ques, ans, 0, 0],
	);
	println!("Flashcard added!")
}

fn clear_database(conn: &Connection) { // Very scary
	println!("IRREVERSIBLE ACTION - CONFIRMATION REQUIRED: Are you sure you want to clear the database? (y/N)"); // Default no
	let input: String = get_user_input();
	if input.to_lowercase() == "y" {
		let _ = conn.execute(
			"DROP TABLE flashcards;",
			params![],
		);
		println!("Database dropped!");
	} else {
		println!("Database not dropped.");
	}
}

// ## Flashcard revision functions ##
fn get_random_flashcard<'a>(list_of_indexes: Vec<usize>, length:usize) -> usize {
	let mut rng = rand::thread_rng();
	println!("Randomising card from 0 to {length}");
	loop {
		let rand_number = rng.gen_range(0..length);
		if list_of_indexes.contains(&rand_number) {
			// Flash card already done. Retry!
		} else {
			return rand_number; // Return random value
		}
	}
}

fn congratulations(flashcard: Flashcard) {
	let accuracy: f64 = (flashcard.correct / (flashcard.correct + flashcard.incorrect)).into();
	println!("Well done! Your accuracy is now {}", accuracy);
}

fn commiserations(flashcard: Flashcard) {
	let accuracy:f64 = (flashcard.correct / (flashcard.correct + flashcard.incorrect)).into();
	println!("Whoops! Your accuracy is now {}", accuracy);
}

fn revision_summary(correct_total : i32, cards_practiced : i32, to_move_up: Vec<i32>, to_move_down: Vec<i32>, subject_name: &str, conn: &Connection) {
	println!();
	println!("Post flashcard breakdown:");
	let percent_accuracy: f64 = (correct_total as f64 / cards_practiced as f64) * 100.0;
	let mut cards= "card";

	if cards_practiced > 1 {
		cards = "cards"; 
	}

	if cards_practiced == 0 {
		println!("No cards practiced!");
	} else {
		println!("You practiced {0} {1}, and got {2} of those correct. That's {3}%!", cards_practiced, cards, correct_total, percent_accuracy);
		println!();
		println!("Learning progress breakdown; ");

		if to_move_up.len() > 0 {
			println!("Cards moving upwards;");
			for &index in to_move_up.iter() {
				let question: Result<String, rusqlite::Error> = conn.query_row(
					format!("SELECT question FROM {} WHERE id = ?1;", subject_name).as_str(),
					params![index],
					|row| row.get(0),
				);
				println!("- {:?}", question);
			}
			println!();
		}

		if to_move_down.len() > 0 {
			println!("Cards moving down;");
			for &index in to_move_down.iter() {
				let question: Result<String, rusqlite::Error> = conn.query_row(
					format!("SELECT question FROM {} WHERE id = ?1;", subject_name).as_str(),
					params![index],
					|row| row.get(0),
				);
				println!("- {:?}", question);
			}
			println!();
		}
	}
}

// ## General functions ##
fn get_user_input() -> String { // New standard for handling user input.
	let mut input: String = String::new();
	let _n = io::stdin().read_line(&mut input).unwrap();
	return input.trim().to_string();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// ## SQLite database ##
	let conn: Connection = Connection::open("flashcards.db")?; // Creates a new database if it doesn't exist or opens it if it does

	// Create new table for subject (Instead of database like previously) if not already present
	let mut subject_name: &str = "business"; // Eventually user input
	let _ = conn.execute(
		format!("CREATE TABLE IF NOT EXISTS {} (
			id INTEGER PRIMARY KEY,
			category INTEGER NOT NULL,
			question TEXT NOT NULL,
			answer TEXT NOT NULL,
			correct INTEGER NOT NULL,
			incorrect INTEGER NOT NULL
		);", subject_name).as_str(), // For category; 0 = weak, 1 = learning, 2 = strong
		params![],
	);

	// Create flashcards loop
	let mut creating_flashcards: bool = true;
	while creating_flashcards == true {
		println!("Would you like to add a flashcard? (y/n)");
		let input: String = get_user_input();
		if input == "y" {
			println!("Enter the question:");
			let ques: String = get_user_input();

			println!("Enter the answer:");
			let ans: String = get_user_input();

			add_new_flashcard(&conn, ques, ans, subject_name);
			/* Creates a new flashcard with the selected question. */
		} else {
			creating_flashcards = false;
		}
	}

	// ## Operational loop ##
	// Declare flashcard variables
	let mut strong_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done well generally
	let mut learning_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done well sometimes
	let mut weak_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done poorly

	// ## Revision loop ##
	// Set to_practice
	let mut to_practice: i32;
	println!("Revise which set? (weak, learning, strong)");
	let input: String = get_user_input();

	if input == "weak" {
		to_practice = 0;
	} else if input == "learning" {
		to_practice = 1;
		println!("WARNING: Unsupported!");
	} else if input == "strong" {
		to_practice = 2;
		println!("WARNING: Unsupported!");
	} else {
		println!("Invalid input. Defaulting to weak flashcards.");
		to_practice = 0;
	};

	// ## Load flashcards from the database ##
	// Select relevant flashcards from the database.
	// Then add them into a vector.
	let query = format!("SELECT id FROM {} WHERE category = ?1;", subject_name);
	let mut stmt: rusqlite::Statement<'_> = conn.prepare(&query)?;
	let ids = stmt.query_map(params![to_practice], |row| row.get(0))?;

	for id_result in ids {
	let id: i32 = id_result?;
		let flashcard: Result<Flashcard, rusqlite::Error> = conn.query_row(
			format!("SELECT * FROM {} WHERE id = ?1;", subject_name).as_str(),
			params![id],
			|row| {
				Ok(Flashcard {
					primary_key: row.get(0)?,
					question: row.get(2)?,
					answer: row.get(3)?,
					correct: row.get(4)?,
					incorrect: row.get(5)?,
				})
			},
		);

		if flashcard.is_err() {
			panic!("Error selecting flashcard from the database.");
		} else {
			let flashcard: Flashcard = flashcard.unwrap();
			if to_practice == 0 {
				weak_flashcards.push(flashcard);
			} else if to_practice == 1 {
				learning_flashcards.push(flashcard);
			} else {
				strong_flashcards.push(flashcard);
			}
		}
	}

	let mut cards_done: usize = 0;
	let mut cards_selected: Vec<usize> = Vec::new(); // Must store primary keys of flashcards chosen to prevent repeats (Could be more efficient?)
	let mut correct: bool = false;
	let mut correct_total: i32 = 0;
	
	// Cards to be moved upwards following this revision session (Doesn't get used if practice_set is strong_flashcards)
	let mut to_move_up: Vec<i32> = Vec::new(); // Stores primary_key
	// Cards to be moved downwards following this revision session (Doesn't get used if practice_set is weak_flashcards)
	let mut to_move_down: Vec<i32> = Vec::new(); // Stores primary_key

	// Determines which flashcard set will be practiced
	let length_of_set: usize;
	if to_practice == 0 {
		length_of_set = weak_flashcards.len();
	} else if to_practice == 1 {
		length_of_set = learning_flashcards.len();
	} else {
		length_of_set = strong_flashcards.len();
	}

	/* **Explanation of below loop**
		- Practiced flashcards are removed from practice_set
		- Flashcards are randomly selected from the original set (e.g. weak_flashcards)
		- Random index is checked against list of already chosen indexes (Stored in cards_selected)
			- If already chosen, rerandomise
			- If not already in cards_selected, return that value
		- Add the provided index to cards_selected
		- Use the provided index to provide a question
		- Check answer and, if needed, get user verification
		- Log accuracy
		- Based on success or lack thereof, place the card into a vector marking it to be moved
			to a new set following the termination of the loop. EXAMPLE;

			if correct;
				to_move_up.push(this_flashcard);
			else
				to_move_down.push(this_flashcard);

			// After loop
			for element in to_move_up
				higher_set.push(to_move_up[element]);
			for element in to_move_down
				lower_set.push(to_move_down[element]);
	 */
	// ## Revision loop ##
	while cards_done < length_of_set {
		// ## Ask a random flashcard question ##
		// Get rand question
		let index_of_question:usize  = get_random_flashcard(cards_selected.clone(), length_of_set);
		cards_selected.push(index_of_question);
		
		let mut flashcard_chosen: Flashcard;
		if to_practice == 0 {
			flashcard_chosen = weak_flashcards[index_of_question].clone();
		} else if to_practice == 1 {  
			flashcard_chosen = learning_flashcards[index_of_question].clone();
		} else {
			flashcard_chosen = strong_flashcards[index_of_question].clone();
		}

		// Display question
		println!("{}",flashcard_chosen.question);

		println!();

		// Both lines required for input handling
		let input: String = get_user_input();

		println!();

		// Compare actual answer and the input
		println!("Your answer: {}", input);
		println!("Actual answer: {}", flashcard_chosen.answer);

		if input.to_lowercase() == flashcard_chosen.answer.to_lowercase() {
			correct = true;			
		} else {
			println!("Was your answer correct? (y/n)");
			let input: String = get_user_input();
			println!();
			//println!("Input: {}", input.trim().to_lowercase());
			if input.to_lowercase() == "y" { // Answer correct
				correct = true;
			} else if input.to_lowercase() == "n" { // Answer correct
				correct = false;
			}
		}
		
		// If cards are correct and this is not the highest tier of cards
		if correct {
			if to_practice != 2 {
				// Can move upwards post revision
				if to_practice == 0 {
					to_move_up.push(weak_flashcards[index_of_question].primary_key);
				} else {
					to_move_up.push(learning_flashcards[index_of_question].primary_key);
				}
			}

			flashcard_chosen.correct += 1;
			correct_total += 1;

			congratulations(flashcard_chosen);
		} else {
			if to_practice != 0 {
				// Can mode downwards post revision
				if to_practice == 1 {
					to_move_down.push(learning_flashcards[index_of_question].primary_key);
				} else {
					to_move_down.push(strong_flashcards[index_of_question].primary_key);
				}
			}

			flashcard_chosen.incorrect += 1;

			commiserations(flashcard_chosen);
		}
		
		cards_done += 1;
	}

	// Post revision summary
	if to_practice == 0 {
		//revision_summary(correct_total, cards_done.try_into().unwrap(), to_move_up.clone(),
		 //to_move_down.clone(), weak_flashcards.clone());
	} else if to_practice == 1 {
		//revision_summary(correct_total, cards_done.try_into().unwrap(), to_move_up.clone(),
		 //to_move_down.clone(), learning_flashcards.clone());
	} else {
		//revision_summary(correct_total, cards_done.try_into().unwrap(), to_move_up.clone(),
		 //to_move_down.clone(), strong_flashcards.clone());
	}

	// ## Post revision card adjustment ##
	// Move cards up/down categories and adjust correct/incorrect values
	// Up
	to_move_up.sort();
	if to_practice == 0 {
		// Move cards upwards to `learning_flashcards`; cat = 1
		// Modern
		for &index in to_move_up.iter() {
			let _ = conn.execute(
				format!("UPDATE {} SET category = 1, correct = correct + 1 WHERE id = ?1;", subject_name).as_str(),
				params![index],
			);
		}
	} else if to_practice == 1 {
		// Move cards upwards to `strong_flashcards`; cat = 2
		for &index in to_move_up.iter() {
			let _ = conn.execute(
				format!("UPDATE {} SET category = 1, correct = correct + 1 WHERE id = ?1;", subject_name).as_str(),
				params![index],
			);
		}
	}

	// Down
	to_move_down.sort();
	if to_practice == 2 {
		// Move cards downwards to `weak_flashcards`
		for &index in to_move_down.iter() {
			let _ = conn.execute(
				format!("UPDATE {} SET category = 0, incorrect = incorrect + 1 WHERE id = ?1;", subject_name).as_str(),
				params![index],
			);
		}
	} else if to_practice == 1 {
		// Move cards downwards to `weak_flashcards`
		for &index in to_move_down.iter() {
			let _ = conn.execute(
				format!("UPDATE {} SET category = 0, incorrect = incorrect + 1 WHERE id = ?1;", subject_name).as_str(),
				params![index],
			);
		}
	}

	// Revision summary goes here!
	revision_summary(correct_total, cards_done as i32, to_move_up, to_move_down, subject_name, &conn);

	Ok(()) // Don't know what this does but the compiler wants it.
}
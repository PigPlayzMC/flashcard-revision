//* For best comment formatting, please use Better Comments extension (VSCode)\

// ## Imports ##
use rand::Rng;
use rusqlite::{params, Connection};
use core::panic;
use std::io;
use chrono::Utc;

// All flashcards follow this structure
#[derive(Clone)]
pub struct Flashcard {
	primary_key: i32, // Primary key of the flashcard in the subject's table (Not the subject table)
	question: String, // Question or front text of the card
	answer: String, // Answer or back text of the card
}

// ## SQLite functions ##
//* Creates new flashcard */
fn add_new_flashcard(conn: &Connection, ques: String, ans: String, subject_name: &str) {
	// This takes inputs and adds it to the correct database, based on the subject.
	let _ = conn.execute(
		format!("INSERT INTO {} (category, question, answer, correct, incorrect) VALUES (?1, ?2, ?3, ?4, ?5);", subject_name).as_str(),
		params![0, ques, ans, 0, 0],
	);
	println!("Flashcard added!")
}

//* Drops ENTIRE table*/
// Broken - Needs fixing (Subject name)
fn clear_database(conn: &Connection, subject_name: &str) {
	//! VERY SCARY - USE WITH CAUTION
	// This clears the specified table. This is irreversible.
	println!("IRREVERSIBLE ACTION - CONFIRMATION REQUIRED: Are you sure you want to remove this subject? (y/N)"); // Default no
	let input: String = get_user_input();
	if input.to_lowercase() == "y" {
		let _ = conn.execute(
			format!("DROP TABLE {};", subject_name).as_str(),
			params![],
		);
		println!("Table cleared!");
	} else {
		println!("Table not cleared.");
	}
}

//* Remove specified flashcard from the subject table */
fn remove_flashcard(conn: &Connection, primary_key: i32, subject_name: &str) {
	let _ = conn.execute(
		format!("DELETE FROM {} WHERE id = ?1;", subject_name).as_str(),
		params![primary_key],
	);
	println!("Flashcard removed!");
}

//* Edit either the answer or question of a specified flashcard */
fn edit_flashcard(conn: &Connection, primary_key: i32, subject_name: &str, field_to_change: i32, new_value: String) {
	// Changes a field on the flashcard (0 = question, 1 = answer)
	if field_to_change == 0 {
		let _ = conn.execute(
			format!("UPDATE {} SET question = ?1 WHERE id = ?2;", subject_name).as_str(),
			params![new_value, primary_key],
		);
	} else if field_to_change == 1 {
		let _ = conn.execute(
			format!("UPDATE {} SET answer = ?1 WHERE id = ?2;", subject_name).as_str(),
			params![new_value, primary_key],
		);
	}
}

fn display_subjects(conn: &Connection) {
	let mut stmt = conn.prepare("SELECT name FROM subjects;").unwrap();
	let subjects = stmt.query_map(params![], |row| {
		Ok(row.get::<_, String>(0)?)
	}).unwrap();

	println!("Subjects:");
	let mut index: i32 = 1;
	for subject in subjects {
		println!("{}. {}", index, subject.unwrap_or("ERR: Failed to load...".to_string()));
		index += 1;
	}

	println!();
	println!("Select subject based on id or type 'new' to create a new subject.");
}

// ## Flashcard revision functions ##
fn get_random_flashcard<'a>(list_of_indexes: Vec<usize>, length: usize) -> usize {
	let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
	println!("Randomising card from 0 to {length}");
	loop {
		let rand_number: usize = rng.gen_range(0..length);
		if list_of_indexes.contains(&rand_number) {
			// Flash card already done. Retry!
		} else {
			return rand_number; // Return random value
		}
	}
}

fn congratulations(flashcard: Flashcard, conn: &Connection, subject_name: &str) {
	// correct/answers
	let correct: Result<i32, rusqlite::Error> = conn.query_row(
		format!("SELECT correct FROM {} WHERE id = ?1;", subject_name).as_str(),
		params![flashcard.primary_key],
		|row| row.get(0),
	);

	let incorrect: Result<i32, rusqlite::Error> = conn.query_row(
		format!("SELECT incorrect FROM {} WHERE id = ?1;", subject_name).as_str(),
		params![flashcard.primary_key],
		|row| row.get(0),
	);

	let correct: f64 = correct.unwrap_or(0) as f64;
	let incorrect: f64 = incorrect.unwrap_or(0) as f64;
	let total_attempts: f64 = correct + incorrect;
	let accuracy: f64;
	if total_attempts == 0.0 {
		accuracy = 1.0;
	} else {
		accuracy = correct / total_attempts;
	}

	println!("Well done! Your accuracy is now {}.", accuracy);
}

fn commiserations(flashcard: Flashcard, conn: &Connection, subject_name: &str) {
	let correct: Result<i32, rusqlite::Error> = conn.query_row(
		format!("SELECT correct FROM {} WHERE id = ?1;", subject_name).as_str(),
		params![flashcard.primary_key],
		|row| row.get(0),
	);

	let incorrect: Result<i32, rusqlite::Error> = conn.query_row(
		format!("SELECT incorrect FROM {} WHERE id = ?1;", subject_name).as_str(),
		params![flashcard.primary_key],
		|row| row.get(0),
	);

	let correct: f64 = correct.unwrap_or(0) as f64;
	let incorrect: f64 = incorrect.unwrap_or(0) as f64;
	let total_attempts = correct + incorrect;
	let accuracy: f64;
	if total_attempts == 0.0 {
		accuracy = 0.0;
	} else {
		accuracy = correct / total_attempts;
	}

	println!("Whoops! Your accuracy is now {}.", accuracy);
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

		println!("Cards moving upwards;");
		if to_move_up.len() > 0 {
			for &index in to_move_up.iter() {
				let question: Result<String, rusqlite::Error> = conn.query_row(
					format!("SELECT question FROM {} WHERE id = ?1;", subject_name).as_str(),
					params![index],
					|row| row.get(0),
				);
				println!("- {}", question.unwrap_or("ERR: Not found...".to_string()));
			}
			println!();
		} else {
			println!("None!")
		}

		println!("Cards moving down;");
		if to_move_down.len() > 0 {
			for &index in to_move_down.iter() {
				let question: Result<String, rusqlite::Error> = conn.query_row(
					format!("SELECT question FROM {} WHERE id = ?1;", subject_name).as_str(),
					params![index],
					|row| row.get(0),
				);
				println!("- {}", question.unwrap_or("ERR: Not found...".to_string()));
			}
			println!();
		} else {
			println!("None!")
		}

		// Update time of last revision
		let now = Utc::now();
		let date: i64 = now.timestamp(); // Seconds since epoch
		let _ = conn.execute(
			format!("UPDATE subjects SET date_last_revised = ?1 WHERE name = ?2;").as_str(),
			params![date, subject_name],
		);
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

	// Storage of tables
	let _ = conn.execute(
		"CREATE TABLE IF NOT EXISTS subjects (
			id INTEGER PRIMARY KEY,
			name TEXT NOT NULL,
			date_last_revised INTEGER NOT NULL
		);", // Stores date as seconds since epoch
		params![],
	);

	// Create new table for subject (Instead of database like previously) if not already present
	display_subjects(&conn);
	let input: String = get_user_input();
	let mut subject_name: String = input.clone(); // Will need to be mut in future

	// Chrono date getting
	let now = Utc::now();
	let date: i64 = now.timestamp(); // Seconds since epoch

	// Add newly created subject to list of subjects
	// Check if subject already exists
	if input == "new".to_owned() {
		println!("Creating new subject. Please enter the name of the subject:");
		subject_name = get_user_input();
		let _ = conn.execute(
			"INSERT INTO subjects (name, date_last_revised) VALUES (?1, ?2);",
			params![subject_name, date],
		);
	} else {
		subject_name = conn.query_row(
			"SELECT name FROM subjects WHERE id = ?1;",
			params![subject_name],
			|row| row.get(0),
		)?;
	}

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

			add_new_flashcard(&conn, ques, ans, subject_name.as_str());
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
	let to_practice: i32;
	println!("Revise which set? (weak, learning, strong)");
	let input: String = get_user_input();

	if input == "weak" {
		to_practice = 0;
	} else if input == "learning" {
		to_practice = 1;
	} else if input == "strong" {
		to_practice = 2;
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
		
		let flashcard_chosen: Flashcard;
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

			let _ = conn.execute(
				format!("UPDATE {} SET correct = correct + 1 WHERE id = ?1;", subject_name).as_str(),
				params![flashcard_chosen.primary_key],
			);
			correct_total += 1;

			congratulations(flashcard_chosen, &conn, subject_name.as_str());
		} else {
			if to_practice != 0 {
				// Can mode downwards post revision
				if to_practice == 1 {
					to_move_down.push(learning_flashcards[index_of_question].primary_key);
				} else {
					to_move_down.push(strong_flashcards[index_of_question].primary_key);
				}
			}

			let _ = conn.execute(
				format!("UPDATE {} SET incorrect = incorrect + 1 WHERE id = ?1;", subject_name).as_str(),
				params![flashcard_chosen.primary_key],
			);

			commiserations(flashcard_chosen, &conn, subject_name.as_str());
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
				format!("UPDATE {} SET category = 1 WHERE id = ?1;", subject_name).as_str(),
				params![index],
			);
		}
	} else if to_practice == 1 {
		// Move cards upwards to `strong_flashcards`; cat = 2
		for &index in to_move_up.iter() {
			let _ = conn.execute(
				format!("UPDATE {} SET category = 2 WHERE id = ?1;", subject_name).as_str(),
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
				format!("UPDATE {} SET category = 0 WHERE id = ?1;", subject_name).as_str(),
				params![index],
			);
		}
	} else if to_practice == 1 {
		// Move cards downwards to `weak_flashcards`
		for &index in to_move_down.iter() {
			let _ = conn.execute(
				format!("UPDATE {} SET category = 0 WHERE id = ?1;", subject_name).as_str(),
				params![index],
			);
		}
	}

	// Revision summary goes here!
	revision_summary(correct_total, cards_done as i32, to_move_up, to_move_down, subject_name.as_str(), &conn);

	Ok(()) // Don't know what this does but the compiler wants it.
}
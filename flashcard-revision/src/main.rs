use rand::Rng;
use std::io;
use chrono::{self, Local, NaiveDate};

// All flashcards follow this structure
#[derive(Clone)]
pub struct Flashcard {
	last_accessed: NaiveDate, // Change when date storage established
	question: String, // Question or front text of the card
	answer: String, // Answer or back text of the card
	correct: i32, // Times the user has gotten the question correct
	incorrect: i32, // Times the user has gotten the question incorrect
}

fn add_new_flashcard(ques: String, ans: String) -> Flashcard {
	// This takes inputs and turns it into a struct
	// Then returns it.
	return Flashcard {
		last_accessed: Local::now().date_naive(), // Doesn't need to be set as it has never been accessed
		question: ques,
		answer: ans,
		correct: 0, // Never answered
		incorrect: 0, // Never answered
	}
}

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

fn revision_summary(correct_total : i32, cards_practiced : i32, to_move_up : Vec<Flashcard>, to_move_down : Vec<Flashcard>) {
	println!("Post flashcard breakdown:");
	let percent_accuracy = correct_total/cards_practiced*100;
	let mut cards= "card";

	if cards_practiced > 1 {
		cards = "cards";
	}
	
	println!("You practiced {0} {1}, and got {2} of those correct. That's {3}%!", cards_practiced, cards, correct_total, percent_accuracy);
	println!();
	println!("Learning progress breakdown; ");

	if to_move_up.len() > 0 {
		println!("Cards moving upwards;");
		for counter in 0..to_move_up.len() {
			println!("{}", to_move_up[counter].question);
		}
		println!();
	}

	if to_move_down.len() > 0 {
		println!("Cards moving down;");
		for counter in 0..to_move_down.len() {
			println!("{}", to_move_down[counter].question);
		}
		println!();
	}
}

fn main() {
	// Declare flashcard variables
	let mut strong_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done well generally
	let mut learning_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done well sometimes
	let mut weak_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done poorly
	let debug_flashcard: Flashcard = add_new_flashcard("DEFAULT FLASHCARD".to_owned(), "PLEASE IGNORE".to_owned());

	// ## Add a flashcard to a subject ##
	let ques: String = "Hello World!".to_owned();
	let ans: String = "Hello World!".to_owned();
	// Add a new flashcard
	weak_flashcards.push(add_new_flashcard(ques, ans));
	// ## Add a flashcard to a subject ##
	let ques: String = "Second!".to_owned();
	let ans: String = "Second!".to_owned();
	// Add a new flashcard
	weak_flashcards.push(add_new_flashcard(ques, ans));

	// ### ### REVISION LOOP ### ###
	let mut to_practice: &str = "weak"; // Set by user need eventually. Can be "weak", "learning", or "strong"
	let mut practice_set: Vec<Flashcard> = Vec::new();
	let mut cards_done: usize = 0;
	let mut cards_selected: Vec<usize> = Vec::new();
	let mut correct: bool = false;
	let mut correct_total = 0;
	
	// Cards to be moved upwards following this revision session (Doesn't get used if practice_set is strong_flashcards)
	let mut to_move_up: Vec<Flashcard> = Vec::new();
	// Cards to be moved downwards following this revision session (Doesn't get used if practice_set is weak_flashcards)
	let mut to_move_down: Vec<Flashcard> = Vec::new();

	// Determines which flashcard set will be practiced.
	if to_practice == "weak" {
		practice_set = weak_flashcards.clone();
	} else if to_practice == "learning" {
		practice_set = learning_flashcards.clone();
	} else {
		practice_set = strong_flashcards.clone();
	}

	let length_of_set: usize = practice_set.len();

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

	// Loop proper
	while cards_done < practice_set.len() {
		// ## Ask a random flashcard question ##
		// Get rand question
		let index_of_question: usize = get_random_flashcard(cards_selected.clone(), length_of_set);
		cards_selected.push(index_of_question);
		
		let mut flashcard_chosen: Flashcard = debug_flashcard.clone();
		if to_practice == "weak" {
			flashcard_chosen = weak_flashcards[index_of_question].clone();
		} else if to_practice == "learning" {
			flashcard_chosen = learning_flashcards[index_of_question].clone();
		} else {
			flashcard_chosen = strong_flashcards[index_of_question].clone();
		}

		// Display question
		println!("{}",flashcard_chosen.question);

		println!();

		// Both lines required for input handling
		let mut input = String::new();
		let _n = io::stdin().read_line(&mut input).unwrap();

		println!();

		// Compare actual answer and the input
		println!("Your answer: {}", input.trim());
		println!("Actual answer: {}", flashcard_chosen.answer);

		flashcard_chosen.last_accessed = Local::now().date_naive(); // EVERYTHING will be a single line!

		if input.trim().to_lowercase() == flashcard_chosen.answer.to_lowercase() {
			correct = true;			
		} else {
			println!("Was your answer correct? (y/n)");
			let mut input = String::new();
			let _n = io::stdin().read_line(&mut input).unwrap();
			println!();
			//println!("Input: {}", input.trim().to_lowercase());
			if input.trim().to_lowercase() == "y" { // Answer correct
				correct = true;
			} else if input.trim().to_lowercase() == "n" { // Answer correct
				correct = false;
			}
		}
		
		// If cards are correct and this is not the highest tier of cards
		if correct {
			if to_practice != "strong" {
				// Can move upwards post revision
				to_move_up.push(flashcard_chosen.clone());
			}

			flashcard_chosen.correct += 1;
			correct_total += 1;

			congratulations(flashcard_chosen);
		} else {
			if to_practice != "weak" {
				// Can mode downwards post revision
				to_move_down.push(flashcard_chosen.clone());
			}

			flashcard_chosen.incorrect += 1;

			commiserations(flashcard_chosen);
		}
		
		cards_done += 1;
	}

	// Post revision summary
	revision_summary(correct_total, cards_done.try_into().unwrap(), to_move_up.clone(), to_move_down.clone());

	// Cards move in their tiers responding to ability
}
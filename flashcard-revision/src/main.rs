use rand::Rng;
use std::io;
use chrono::{self, Local, NaiveDate};

// All flashcards follow this structure
#[derive(Clone)]
pub struct Flashcard {
	subject: i32, // Which subject these fall into
	last_accessed: NaiveDate, // Change when date storage established
	question: String, // Question or front text of the card
	answer: String, // Answer or back text of the card
	correct: i32, // Times the user has gotten the question correct
	incorrect: i32, // Times the user has gotten the question incorrect
}

fn add_new_flashcard(sub:i32, ques: String, ans: String) -> Flashcard{
	// This takes inputs and turns it into a struct
	// Then returns it.
	return Flashcard {
		subject: sub,
		last_accessed: Local::now().date_naive(), // Doesn't need to be set as it has never been accessed
		question: ques,
		answer: ans,
		correct: 0, // Never answered
		incorrect: 0, // Never answered
	}
}

fn get_random_flashcard<'a>(card_set: &'a mut Vec<Flashcard>) -> usize {
	let mut rng = rand::thread_rng();
	let length = card_set.len();
	let rand_number = rng.gen_range(0..length);
	return rand_number; // Return random value
}

fn congratulations(flashcard: Flashcard) {
	let accuracy: f64 = (flashcard.correct / (flashcard.correct + flashcard.incorrect)).into();
	println!("Well done! Your accuracy is now {}", accuracy);
}

fn comiserations(flashcard: Flashcard) {
	let accuracy:f64 = (flashcard.correct / (flashcard.correct + flashcard.incorrect)).into();
	println!("Whoops! Your accuracy is now {}", accuracy);
}

fn main() {
	// Declare subjects to 'store' flashcards in
	let mut subjects: Vec<String> = Vec::new(); // Store flashcards

	// Declare flashcard variables
	let mut strong_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done well generally
	let mut learning_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done well sometimes
	let mut weak_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done poorly

	// ## Add a flashcard to a subject ##
	let sub = 0;
	let ques: String = "Hello World!".to_owned();
	let ans: String = "Hello World!".to_owned();
	// Add a new flashcard
	weak_flashcards.push(add_new_flashcard(sub, ques, ans));

	// ## Ask a random flashcard question ##
	// Get rand question
	let index_of_question = get_random_flashcard(&mut weak_flashcards);
	let mut question_to_ask = weak_flashcards[index_of_question].clone();
	let practice_set = "weak"; // Dependent on wether weak, mid, or stong being practiced.
	// Display question
	println!("{}",question_to_ask.question);

	println!();

	// Both lines required for input handling
	let mut input = String::new();
	let _n = io::stdin().read_line(&mut input).unwrap();

	println!();

	// Compare actual answer and the input
	println!("Your answer: {}", input.trim());
	println!("Actual answer: {}", question_to_ask.answer);

	question_to_ask.last_accessed = Local::now().date_naive(); // EVERYTHING will be a single line!

	if input.trim() == question_to_ask.answer {
		println!("Correct!");
		question_to_ask.correct += 1;

		if practice_set == "weak" {
			println!("Moving to learning...");
			weak_flashcards.swap_remove(index_of_question);
			learning_flashcards.push(question_to_ask.clone());
		} else if practice_set == "learning" {
			println!("Moving to strong...");
			learning_flashcards.swap_remove(index_of_question);
			strong_flashcards.push(question_to_ask.clone());
		}

		congratulations(question_to_ask);
	} else {
		println!("Was your answer correct? (y/n)");
		let mut input = String::new();
		let _n = io::stdin().read_line(&mut input).unwrap();
		println!("Input: {}", input.trim().to_lowercase());
		if input.trim().to_lowercase() == "y" { // Answer correct
			question_to_ask.correct += 1;
			
			if practice_set == "weak" {
				println!("Moving to learning...");
				weak_flashcards.swap_remove(index_of_question);
				learning_flashcards.push(question_to_ask.clone());
			} else if practice_set == "learning" {
				println!("Moving to strong...");
				learning_flashcards.swap_remove(index_of_question);
				strong_flashcards.push(question_to_ask.clone());
			}

			congratulations(question_to_ask);
		} else if input.trim().to_lowercase() == "n" { // Answer correct
			question_to_ask.incorrect += 1;

			if practice_set == "learning" {
				println!("Moving to weak...");
				learning_flashcards.swap_remove(index_of_question);
				weak_flashcards.push(question_to_ask.clone());
			} else if practice_set == "strong" {
				println!("Moving to learning...");
				strong_flashcards.swap_remove(index_of_question);
				weak_flashcards.push(question_to_ask.clone());
			}

			comiserations(question_to_ask);
		}
	}

	// Debug pls remove owo
	for counter in 0..weak_flashcards.len() {
		println!("Weak: {}", weak_flashcards[counter].question);
	}
	for counter in 0..learning_flashcards.len() {
		println!("Learning: {}", learning_flashcards[counter].question);
	}
	for counter in 0..strong_flashcards.len() {
		println!("Strong: {}", strong_flashcards[counter].question);
	}
}
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

fn main() {
	// Declare subjects to 'store' flashcards in
	let mut subjects: Vec<String> = Vec::new(); // Store flashcards

	// Declare flashcard variables
	let mut strong_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done well generally
	let mut learning_flashcard: Vec<Flashcard> = Vec::new(); // Flashcards done well sometimes
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

	let mut input = String::new();
	let _n = io::stdin().read_line(&mut input).unwrap();

	println!();

	// Compare actual answer and the input
	println!("Your answer: {}", input.trim());
	println!("Actual answer: {}", question_to_ask.answer);

	if input.trim() == question_to_ask.answer {
		println!("Correct!");
		question_to_ask.correct += 1;
	} else {
		println!("Was your answer correct? (y/n)");
		let _n = io::stdin().read_line(&mut input).unwrap();
		if input.trim().to_lowercase() == "y" {
			question_to_ask.correct += 1;
			
			if practice_set == "weak" {
				weak_flashcards.swap_remove(index_of_question);
				learning_flashcard.push(question_to_ask.clone());
			} else if practice_set == "learning" {
				learning_flashcard.swap_remove(index_of_question);
				strong_flashcards.push(question_to_ask.clone());
			}
		} else if input.trim().to_lowercase() == "n" {
			question_to_ask.incorrect += 1;

			if practice_set == "learning" {
				learning_flashcard.swap_remove(index_of_question);
				weak_flashcards.push(question_to_ask.clone());
			} else if practice_set == "strong" {
				strong_flashcards.swap_remove(index_of_question);
				weak_flashcards.push(question_to_ask.clone());
			}
		}
	}

	question_to_ask.last_accessed = Local::now().date_naive(); // EVERYTHING will be a single line!
}
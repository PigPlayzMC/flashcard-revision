use rand::prelude::*;

// All flashcards follow this structure
pub struct Flashcard {
    subject: i32, // Which subject these fall into
    last_accessed: i32, //Change when date storage established
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
        last_accessed: 0, // Doesn't need to be set as it has never been accessed
        question: ques,
        answer: ans,
        correct: 0, // Never answered
        incorrect: 0, // Never answered
    }
}

fn get_random_flashcard(card_set:Vec<Flashcard>) -> Flashcard {
    return card_set[0]; // Return random value [NOT IMPLEMENTED]
}

fn main() {
    // Declare subjects to 'store' flashcards in
    let mut subjects: Vec<String> = Vec::new(); // Store flashcards

    // Declare flashcard variables
    let mut strong_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done well generally
    let mut learning_flashcard: Vec<Flashcard> = Vec::new(); // Flashcards done well sometimes
    let mut weak_flashcards: Vec<Flashcard> = Vec::new(); // Flashcards done poorly

    

    // This is the current flashcard number which determines which flashcard is used.
    let mut flashcard: i32 = 0;

    // Add a flashcard to a subject
    let sub = 0;
    let ques: String = "Hello World!".to_owned();
    let ans: String = "Hello World!".to_owned();

    // Add a new flashcard
    weak_flashcards.push(add_new_flashcard(sub, ques, ans));

    // Ask a random flashcard question
    let question_to_ask = get_random_flashcard(weak_flashcards);

}

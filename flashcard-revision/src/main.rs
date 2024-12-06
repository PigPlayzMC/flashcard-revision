fn add_new_flashcard() {
    // This takes inputs and turns it into a struct
    // Then returns it.
}

fn main() {
    // Declare subjects to 'store' flashcards in
    let mut subjects = Vec::new(); // Store flashcards

    // Declare flashcard variables
    let mut strong_flashcards = Vec::new(); // Flashcards done well generally
    let mut learning_flashcard = Vec::new(); // Flashcards done well sometimes
    let mut weak_flashcards = Vec::new(); // Flashcards done poorly

    // All flashcards follow this structure
    pub struct Flashcard {
        catagory: i32, // Which subject these fall into
        last_accessed: i32, //Change when date storage established
        question: String, // Question or front text of the card
        answer: String, // Answer or back text of the card
        correct: i32, // Times the user has gotten the question correct
        incorrect: i32, // Times the user has gotten the question incorrect
    }

    strong_flashcards.push(flashcard);
}

use macroquad::prelude::*; // Handles window display

use rusqlite::{ // Handles SQLite database
	Connection,
	params};

async fn loading_screen() {
	debug!("Loading...");
	clear_background(Color::from_rgba(0, 0, 0, 1));
	draw_text("Loading...", screen_width() / 2.0 - 40.0, screen_height() / 2.0, 50.0, WHITE);
	next_frame().await;
}

#[macroquad::main("Flashcard Revision")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Display loading screen
	loading_screen().await;

	// ## SQLite database ##
	let conn: Connection = Connection::open("flashcards.db")?; // Creates a new database if it doesn't exist or opens it if it does

	// Storage of tables
	let _ = conn.execute(
		"CREATE TABLE IF NOT EXISTS subjects (
			id INTEGER PRIMARY KEY,
			name TEXT NOT NULL,
			date_weak_revised INTEGER NOT NULL,
			date_learning_revised INTEGER NOT NULL,
			date_strong_revised INTEGER NOT NULL,
		);", // Stores date as seconds since epoch
		params![],
	);

	// ## Window settings ##
	// General colours
	let background_colour: Color = Color::from_rgba(88, 138, 85, 1);
	////let text_colour = todo!();

	// Card colours
	////let weak_colour = todo!();
	////let learning_colour = todo!();
	////let strong_colour = todo!();

	debug!("Main loop reached...");
    loop {
        clear_background(background_colour);

        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        ////draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);


		// End section (Nothing past this point please)
        next_frame().await;
    }
	
	////Ok(())
}